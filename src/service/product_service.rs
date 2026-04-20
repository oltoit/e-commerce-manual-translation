use diesel::{Connection, PgConnection};
use validator::Validate;
use crate::api::controller::pagination::Pagination;
use crate::api::dto::product_dto::{CreateProductDto, UpdateProductDto};
use crate::dao::product_repository;
use crate::entity::product::{NewProduct, ProductWithUser, UpdateProduct};
use crate::errors::error_enum::{ErrorsEnum, DTO_NOT_VALID_ERROR_MSG, PRODUCT_NOT_FOUND_MSG};
use crate::security::auth_context_holder::AuthUser;
use crate::service::currency_conversion_service;
use crate::service::currency_conversion_service::SRC_CURRENCY;

pub fn get_products_with_users(connection: &mut PgConnection, auth_user: &AuthUser, pagination: &Pagination) -> Result<(Vec<ProductWithUser>, i64), ErrorsEnum> {
    if !auth_user.role.has_user_permission() { return Err(ErrorsEnum::Forbidden); }

    match product_repository::get_all_products_with_user(connection, pagination) {
        Ok(results) => Ok(results),
        Err(e) => match e {
            diesel::result::Error::NotFound => Err(ErrorsEnum::NoPropertyError("There was a problem with the sort order".to_string())),
            _ => Err(ErrorsEnum::DieselError(e.to_string())),
        }
    }
}

pub fn get_product_with_user_by_id(connection: &mut PgConnection, auth_user: &AuthUser, product_id: i64) -> Result<ProductWithUser, ErrorsEnum> {
    if !auth_user.role.has_user_permission() { return Err(ErrorsEnum::Forbidden); }

    match product_repository::get_by_id_with_user(connection, product_id) {
        Ok(product) => Ok(product),
        Err(_) => Err(ErrorsEnum::NotFound(PRODUCT_NOT_FOUND_MSG.to_string()))
    }
}

pub async fn create_product(connection: &mut PgConnection, auth_user: &AuthUser, mut create_product: CreateProductDto) -> Result<ProductWithUser, ErrorsEnum> {
    if !auth_user.role.has_user_permission() { return Err(ErrorsEnum::Forbidden); }
    if create_product.validate().is_err() { return Err(ErrorsEnum::DTONotValid(DTO_NOT_VALID_ERROR_MSG.to_string())); }

    if create_product.currency != SRC_CURRENCY {
        create_product.price = currency_conversion_service::convert_currency_to_euro(
            create_product.currency.as_str(),
            create_product.price
        ).await?;
    }

    let new_product = NewProduct::from_dto(&create_product, auth_user);

    connection.transaction(move |conn| {
        match product_repository::insert_return_with_user(conn, new_product) {
            Ok(product) => Ok(product),
            Err(_) => Err(ErrorsEnum::CreationError("error creating Product".to_string()))
        }
    })
}

pub async fn update_product(connection: &mut PgConnection, auth_user: &AuthUser, mut update_product: UpdateProductDto, product_id: i64) -> Result<ProductWithUser, ErrorsEnum> {
    if !auth_user.role.has_user_permission() { return Err(ErrorsEnum::Forbidden); }
    if update_product.validate().is_err() { return Err(ErrorsEnum::DTONotValid(DTO_NOT_VALID_ERROR_MSG.to_string())); }

    if update_product.currency != SRC_CURRENCY {
        update_product.price = currency_conversion_service::convert_currency_to_euro(
            update_product.currency.as_str(),
            update_product.price
        ).await?;
    }

    let update_product = UpdateProduct::from_dto(&update_product, auth_user);

    connection.transaction(move |conn| {
        if !can_mutate_product(conn, auth_user, product_id)? {
            return Err(ErrorsEnum::Forbidden);
        }

        match product_repository::update_return_with_user(conn, update_product, product_id) {
            Ok(product) => Ok(product),
            Err(_) => Err(ErrorsEnum::UpdateError("error updating Product".to_string()))
        }
    })
}

pub fn delete_product(connection: &mut PgConnection, auth_user: &AuthUser, product_id: i64) -> Result<usize, ErrorsEnum> {
    if !auth_user.role.has_user_permission() { return Err(ErrorsEnum::Forbidden); }

    connection.transaction(move |conn| {
        if !can_mutate_product(conn, auth_user, product_id)? {
            return Err(ErrorsEnum::Forbidden);
        }

        match product_repository::delete(conn, product_id) {
            Ok(deleted) => {
                if deleted <= 0 { return Err(ErrorsEnum::NotFound(PRODUCT_NOT_FOUND_MSG.to_string())); }
                Ok(deleted)
            },
            Err(_) => Err(ErrorsEnum::DeletionError("error deleting Product".to_string()))
        }
    })
}

fn can_mutate_product(connection: &mut PgConnection, auth_user: &AuthUser, product_id: i64) -> Result<bool, ErrorsEnum> {
    let is_admin = auth_user.role.has_admin_permission();
    let is_owner = match product_repository::get_by_id(connection, product_id) {
        Ok(product) => product.userid == auth_user.id,
        Err(_) => return Err(ErrorsEnum::NotFound(PRODUCT_NOT_FOUND_MSG.to_string()))
    };
    Ok(is_admin || is_owner)
}