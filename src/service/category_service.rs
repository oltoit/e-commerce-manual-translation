use diesel::{Connection, PgConnection};
use crate::outbound::dao::{category_repository, product_repository};
use crate::shared::auth::auth_user::AuthUser;
use crate::shared::entity::category::{Category, NewCategory, UpdateCategory};
use crate::shared::errors::error_enum::{ErrorsEnum, CATEGORY_NOT_FOUND_MSG, PRODUCT_NOT_FOUND_MSG};


pub fn get_categories(connection: &mut PgConnection, auth_user: &AuthUser) -> Result<Vec<Category>, ErrorsEnum> {
    if !auth_user.role.has_user_permission() { return Err(ErrorsEnum::Forbidden); }

    let result = category_repository::get_all_categories(connection);

    match result {
        Ok(categories) => Ok(categories),
        Err(_) => Err(ErrorsEnum::NotFound(CATEGORY_NOT_FOUND_MSG.to_string()))
    }
}

pub fn get_category_by_id(connection: &mut PgConnection, auth_user: &AuthUser, category_id: i64) -> Result<Category, ErrorsEnum>{
    if !auth_user.role.has_user_permission() { return Err(ErrorsEnum::Forbidden); }

    let result = category_repository::get_by_id(connection, category_id);

    match result {
        Ok(category) => Ok(category),
        Err(_) => Err(ErrorsEnum::NotFound(CATEGORY_NOT_FOUND_MSG.to_string()))
    }
}

pub fn create_category(connection: &mut PgConnection, auth_user: &AuthUser, new_category: NewCategory) -> Result<Category, ErrorsEnum> {
    if !auth_user.role.has_admin_permission() { return Err(ErrorsEnum::Forbidden); }

    connection.transaction(move |conn| {
        match category_repository::insert(conn, new_category) {
            Ok(category) => Ok(category),
            Err(_) => Err(ErrorsEnum::CreationError("error creating Category".to_string()))
        }
    })
}

pub fn update_category(connection: &mut PgConnection, auth_user: &AuthUser, category_id: i64, update_category: UpdateCategory) -> Result<Category, ErrorsEnum> {
    if !auth_user.role.has_admin_permission() { return Err(ErrorsEnum::Forbidden); }

    connection.transaction(move |conn| {
        match category_repository::get_by_id(conn, category_id) {
            Ok(_) => (),
            Err(_) => return Err(ErrorsEnum::NotFound(CATEGORY_NOT_FOUND_MSG.to_string()))
        };

        match category_repository::update(conn, update_category, category_id) {
            Ok(category) => Ok(category),
            Err(_) => Err(ErrorsEnum::UpdateError("error updating Category".to_string()))
        }
    })
}

pub fn delete_category(connection: &mut PgConnection, auth_user: &AuthUser, category_id: i64) -> Result<usize, ErrorsEnum> {
    if !auth_user.role.has_admin_permission() { return Err(ErrorsEnum::Forbidden); }

    connection.transaction(move |conn| {
        match category_repository::delete(conn, category_id) {
            Ok(count) => {
                if count <= 0 { return Err(ErrorsEnum::NotFound(CATEGORY_NOT_FOUND_MSG.to_string())); }
                Ok(count)
            },
            Err(_) => Err(ErrorsEnum::DeletionError("error deleting Category".to_string()))
        }
    })
}

pub fn get_category_for_product(connection: &mut PgConnection, auth_user: &AuthUser, product_id: i64) -> Result<Vec<Category>, ErrorsEnum> {
    if !auth_user.role.has_user_permission() { return Err(ErrorsEnum::Forbidden); }

    match product_repository::get_categories_for_product(connection, product_id) {
        Ok(categories) => Ok(categories),
        Err(_) => Err(ErrorsEnum::NotFound(PRODUCT_NOT_FOUND_MSG.to_string()))
    }
}