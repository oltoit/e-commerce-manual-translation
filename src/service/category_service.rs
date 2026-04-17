use diesel::{Connection, PgConnection};
use crate::api::dto::category_dto::{CreateCategoryDto, UpdateCategoryDto};
use crate::dao::category_repository;
use crate::entity::category::{Category, NewCategory, UpdateCategory};
use crate::errors::error_enums::ErrorsEnum;
use crate::security::auth_context_holder::AuthUser;
// TODO: make the functions that mutate db transactional with PgConnection.transaction()

const CATEGORY_NOT_FOUND_MSG: &'static str = "category not found";

pub fn get_categories(connection: &mut PgConnection, auth_user: &AuthUser) -> Result<Vec<Category>, ErrorsEnum> {
    if !auth_user.role.has_user_permission() { return Err(ErrorsEnum::Forbidden); }

    let result = category_repository::get_all_categories(connection);

    match result {
        Ok(categories) => Ok(categories),
        Err(_) => Err(ErrorsEnum::NotFound(CATEGORY_NOT_FOUND_MSG.to_string()))
    }
}

pub fn get_category_by_id(connection: &mut PgConnection, auth_user: &AuthUser, id: i64) -> Result<Category, ErrorsEnum>{
    if !auth_user.role.has_user_permission() { return Err(ErrorsEnum::Forbidden); }

    let result = category_repository::get_by_id(connection, id);

    match result {
        Ok(category) => Ok(category),
        Err(_) => Err(ErrorsEnum::NotFound(CATEGORY_NOT_FOUND_MSG.to_string()))
    }
}

pub fn get_subcategories_for_id(connection: &mut PgConnection, auth_user: &AuthUser, id: i64) -> Result<Vec<Category>, ErrorsEnum> {
    if !auth_user.role.has_user_permission() { return Err(ErrorsEnum::Forbidden); }

    let result = category_repository::get_subcategories(connection, id);

    match result {
        Ok(categories) => Ok(categories),
        Err(_) => Err(ErrorsEnum::NotFound(CATEGORY_NOT_FOUND_MSG.to_string()))
    }
}

pub fn create_category(connection: &mut PgConnection, auth_user: &AuthUser, create_category: CreateCategoryDto) -> Result<Category, ErrorsEnum> {
    if !auth_user.role.has_admin_permission() { return Err(ErrorsEnum::Forbidden); }
    let new_category = NewCategory::from(&create_category);

    connection.transaction(move |conn| {
        let result = category_repository::insert(conn, new_category);

        match result {
            Ok(category) => Ok(category),
            Err(_) => Err(ErrorsEnum::CreationError("error creating Category".to_string()))
        }
    })
}

pub fn update_category(connection: &mut PgConnection, auth_user: &AuthUser, id: i64, update_category: UpdateCategoryDto) -> Result<Category, ErrorsEnum> {
    if !auth_user.role.has_admin_permission() { return Err(ErrorsEnum::Forbidden); }
    let update_category = UpdateCategory::from(&update_category);

    connection.transaction(move |conn| {
        match category_repository::get_by_id(conn, id) {
            Ok(_) => (),
            Err(_) => return Err(ErrorsEnum::NotFound(CATEGORY_NOT_FOUND_MSG.to_string()))
        };

        match category_repository::update(conn, update_category, id) {
            Ok(category) => Ok(category),
            Err(_) => Err(ErrorsEnum::UpdateError("error updating Category".to_string()))
        }
    })
}

pub fn delete_category(connection: &mut PgConnection, auth_user: &AuthUser, id: i64) -> Result<usize, ErrorsEnum> {
    if !auth_user.role.has_admin_permission() { return Err(ErrorsEnum::Forbidden); }

    connection.transaction(move |conn| {
        match category_repository::delete(conn, id) {
            Ok(count) => {
                if count <= 0 {
                    return Err(ErrorsEnum::NotFound(CATEGORY_NOT_FOUND_MSG.to_string()));
                } else {
                    return Ok(count);
                }
            },
            Err(_) => Err(ErrorsEnum::DeletionError("error deleting Category".to_string()))
        }
    })
}