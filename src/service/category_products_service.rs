use diesel::{Connection, PgConnection};
use crate::api::controller::pagination::Pagination;
use crate::dao::{category_repository, product_category_repository, product_repository, user_repository};
use crate::entity::product::ProductWithUser;
use crate::entity::product_category_relation::ProductCategoryRelation;
use crate::errors::error_enum::{ErrorsEnum, CATEGORY_NOT_FOUND_MSG, PRODUCT_NOT_FOUND_MSG, USER_NOT_FOUND_MSG};
use crate::security::auth_context_holder::AuthUser;
use crate::service::auth_helper::can_mutate_product;

pub fn get_products_for_category(connection: &mut PgConnection, auth_user: &AuthUser, pagination: &Pagination, category_id: i64) -> Result<(Vec<ProductWithUser>, i64), ErrorsEnum> {
    if !auth_user.role.has_user_permission() { return Err(ErrorsEnum::Forbidden); }

    connection.transaction(move |conn| {
        category_repository::get_by_id(conn, category_id).map_err(|_| ErrorsEnum::NotFound(CATEGORY_NOT_FOUND_MSG.to_string()))?;

        let (products, total_elements) = match product_category_repository::get_all_products_for_categories_recursive(conn, pagination, category_id) {
            Ok(products) => products,
            Err(_) => return Err(ErrorsEnum::DieselError("error getting products for category".to_string()))
        };

        Ok((products, total_elements))
    })
}

pub fn add_product_to_category(connection: &mut PgConnection, auth_user: &AuthUser, category_id: i64, product_id: i64) -> Result<ProductWithUser, ErrorsEnum> {
    if !auth_user.role.has_user_permission() { return Err(ErrorsEnum::Forbidden); }

    connection.transaction(move |conn| {
        category_repository::get_by_id(conn, category_id).map_err(|_| ErrorsEnum::NotFound(CATEGORY_NOT_FOUND_MSG.to_string()))?;
        let product = product_repository::get_by_id(conn, product_id).map_err(|_| ErrorsEnum::NotFound(PRODUCT_NOT_FOUND_MSG.to_string()))?;

        if !can_mutate_product(auth_user, &product) { return Err(ErrorsEnum::Forbidden); }

        if product_category_repository::get_product_for_category(conn, category_id, product_id).is_ok() {
            return Err(ErrorsEnum::ProductCategoryError(format!("product '{}' is already in category '{}'", product_id, category_id)));
        }

        let relation = ProductCategoryRelation { productid: product_id, categoryid: category_id };
        match product_category_repository::add_category_to_product(conn, relation) {
            Ok(product) => product,
            Err(_) => return Err(ErrorsEnum::CreationError(format!("error adding product '{}' to category '{}'", product_id, category_id)))
        };

        let user = user_repository::get_by_id(conn, product.userid).map_err(|_| ErrorsEnum::NotFound(USER_NOT_FOUND_MSG.to_string()))?;
        Ok(ProductWithUser::from(product, user))
    })
}

pub fn remove_product_from_category(connection: &mut PgConnection, auth_user: &AuthUser, category_id: i64, product_id: i64) -> Result<(), ErrorsEnum> {
    if !auth_user.role.has_user_permission() { return Err(ErrorsEnum::Forbidden); }

    connection.transaction(move |conn| {
        category_repository::get_by_id(conn, category_id).map_err(|_| ErrorsEnum::NotFound(CATEGORY_NOT_FOUND_MSG.to_string()))?;
        let product = product_repository::get_by_id(conn, product_id).map_err(|_| ErrorsEnum::NotFound(PRODUCT_NOT_FOUND_MSG.to_string()))?;

        if !can_mutate_product(auth_user, &product) { return Err(ErrorsEnum::Forbidden); }

        if product_category_repository::get_product_for_category(conn, category_id, product_id).is_err() {
            return Err(ErrorsEnum::ProductCategoryError(format!("product '{}' is not in category '{}'", product_id, category_id)));
        }

        let relation = ProductCategoryRelation { productid: product_id, categoryid: category_id };
        match product_category_repository::remove_category_from_product(conn, relation) {
            Ok(count) => {
                if count <= 0 {
                    return Err(ErrorsEnum::DeletionError(format!("error removing product '{}' from category '{}'", product_id, category_id)));
                }
            },
            Err(_) => return Err(ErrorsEnum::CreationError(format!("error removing product '{}' from category '{}'", product_id, category_id)))
        };
        Ok(())
    })
}