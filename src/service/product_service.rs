use diesel::{Connection, PgConnection};
use crate::api::controller::pagination::Pagination;
use crate::outbound::adapter::currency_conversion;
use crate::outbound::adapter::currency_conversion::SRC_CURRENCY;
use crate::outbound::dao::product_repository;
use crate::shared::entity::product::{NewProduct, ProductWithUser, UpdateProduct};
use crate::shared::errors::error_enum::{ErrorsEnum, PRODUCT_NOT_FOUND_MSG};
use crate::service::auth_helper::can_mutate_product_by_id;
use crate::shared::auth::auth_user::AuthUser;

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

pub async fn create_product<'a>(
    connection: &mut PgConnection,
    auth_user: &AuthUser,
    mut new_product: NewProduct<'a>,
    currency: &str
) -> Result<ProductWithUser, ErrorsEnum> {
    if !auth_user.role.has_user_permission() { return Err(ErrorsEnum::Forbidden); }

    if currency != SRC_CURRENCY {
        new_product.price = currency_conversion::convert_currency_to_euro(
            currency,
            new_product.price
        ).await?;
    }

    connection.transaction(move |conn| {
        match product_repository::insert_return_with_user(conn, new_product) {
            Ok(product) => Ok(product),
            Err(_) => Err(ErrorsEnum::CreationError("error creating Product".to_string()))
        }
    })
}

pub async fn update_product<'a>(
    connection: &mut PgConnection,
    auth_user: &AuthUser,
    mut update_product: UpdateProduct<'a>,
    currency: &str,
    product_id: i64
) -> Result<ProductWithUser, ErrorsEnum> {
    if !auth_user.role.has_user_permission() { return Err(ErrorsEnum::Forbidden); }

    if currency != SRC_CURRENCY {
        update_product.price = currency_conversion::convert_currency_to_euro(
            currency,
            update_product.price
        ).await?;
    }

    connection.transaction(move |conn| {
        if !can_mutate_product_by_id(conn, auth_user, product_id)? {
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
        if !can_mutate_product_by_id(conn, auth_user, product_id)? {
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