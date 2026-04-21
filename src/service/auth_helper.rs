use diesel::PgConnection;
use crate::dao::product_repository;
use crate::entity::product::Product;
use crate::errors::error_enum::{ErrorsEnum, PRODUCT_NOT_FOUND_MSG};
use crate::security::auth_context_holder::AuthUser;

pub fn can_mutate_product_by_id(connection: &mut PgConnection, auth_user: &AuthUser, product_id: i64) -> Result<bool, ErrorsEnum> {
    let is_admin = auth_user.role.has_admin_permission();
    let is_owner = match product_repository::get_by_id(connection, product_id) {
        Ok(product) => product.userid == auth_user.id,
        Err(_) => return Err(ErrorsEnum::NotFound(PRODUCT_NOT_FOUND_MSG.to_string()))
    };
    Ok(is_admin || is_owner)
}

pub fn can_mutate_product(auth_user: &AuthUser, product: &Product) -> bool {
    let is_admin = auth_user.role.has_admin_permission();
    let is_owner = product.userid == auth_user.id;
    is_admin || is_owner
}