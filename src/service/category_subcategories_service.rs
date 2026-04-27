use diesel::{Connection, PgConnection};
use crate::dao::category_repository;
use crate::entity::category::{Category, UpdateCategory};
use crate::errors::error_enum::{ErrorsEnum, CATEGORY_NOT_FOUND_MSG, SUBCATEGORY_UPDATE_ERROR_MSG};
use crate::security::auth_context_holder::AuthUser;

pub fn get_subcategories_for_category(connection: &mut PgConnection, auth_user: &AuthUser, id: i64) -> Result<Vec<Category>, ErrorsEnum> {
    if !auth_user.role.has_user_permission() { return Err(ErrorsEnum::Forbidden); }

    match category_repository::get_subcategories(connection, id) {
        Ok(categories) => Ok(categories),
        Err(_) => Err(ErrorsEnum::NotFound(CATEGORY_NOT_FOUND_MSG.to_string()))
    }
}

pub fn add_subcategory_to_category(connection: &mut PgConnection, auth_user: &AuthUser, category_id: i64, subcategory_id: i64) -> Result<Category, ErrorsEnum> {
    if !auth_user.role.has_admin_permission() { return Err(ErrorsEnum::Forbidden); }

    connection.transaction(move |conn| {
        let parent_category = match category_repository::get_by_id(conn, category_id) {
            Ok(category) => category,
            Err(_) => return Err(ErrorsEnum::NotFound(CATEGORY_NOT_FOUND_MSG.to_string()))
        };

        let subcategory = match category_repository::get_by_id(conn, subcategory_id) {
            Ok(category) => category,
            Err(_) => return Err(ErrorsEnum::NotFound(CATEGORY_NOT_FOUND_MSG.to_string()))
        };

        if subcategory.parentid.is_some_and(|parent| parent == parent_category.id) {
            return Err(ErrorsEnum::CategoryAssociationError(format!("category {} already contains subcategory {}", category_id, subcategory_id )));
        };

        let mut updated_category = UpdateCategory::from_category(&subcategory);
        updated_category.parentid = Some(parent_category.id);

        match category_repository::update(conn, updated_category, subcategory_id) {
            Ok(_) => (),
            Err(_) => return Err(ErrorsEnum::UpdateError(SUBCATEGORY_UPDATE_ERROR_MSG.to_string()))
        };

        Ok(parent_category)
    })
}

pub fn delete_subcategory_from_category(connection: &mut PgConnection, auth_user: &AuthUser, category_id: i64, subcategory_id: i64) -> Result<Category, ErrorsEnum> {
    if !auth_user.role.has_admin_permission() { return Err(ErrorsEnum::Forbidden); }

    connection.transaction(move |conn| {
        let parent_category = match category_repository::get_by_id(conn, category_id) {
            Ok(category) => category,
            Err(_) => return Err(ErrorsEnum::NotFound(CATEGORY_NOT_FOUND_MSG.to_string()))
        };

        let subcategory = match category_repository::get_by_id(conn, subcategory_id) {
            Ok(category) => category,
            Err(_) => return Err(ErrorsEnum::NotFound(CATEGORY_NOT_FOUND_MSG.to_string()))
        };

        if subcategory.parentid.is_none_or(|parent| parent != parent_category.id) {
            return Err(ErrorsEnum::CategoryAssociationError(format!("category {} does not contain subcategory {}", category_id, subcategory_id )));
        };

        let mut updated_category = UpdateCategory::from_category(&subcategory);
        updated_category.parentid = None;

        match category_repository::update(conn, updated_category, subcategory_id) {
            Ok(subcategory) => Ok(subcategory),
            Err(_) => Err(ErrorsEnum::UpdateError(SUBCATEGORY_UPDATE_ERROR_MSG.to_string()))
        }
    })
}