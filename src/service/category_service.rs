use diesel::PgConnection;
use crate::dao::category_repository::{get_all_categories, get_by_id, get_subcategories};
use crate::entity::category::Category;
use crate::security::auth_context_holder::AuthUser;
// TODO: make the functions that mutate db transactional with PgConnection.transaction()

// TODO: add auth check to every function
pub fn get_categories(connection: &mut PgConnection, auth_user: &AuthUser) -> Option<Vec<Category>> {


    let result = get_all_categories(connection);

    match result {
        Ok(categories) => Some(categories),
        Err(_) => None
    }
}

pub fn get_category_by_id(connection: &mut PgConnection, id: i64) -> Option<Category> {
    let result = get_by_id(connection, id);

    match result {
        Ok(category) => Some(category),
        Err(_) => None
    }
}

pub fn get_subcategories_for_id(connection: &mut PgConnection, id: i64) -> Option<Vec<Category>> {
    let result = get_subcategories(connection, id);

    match result {
        Ok(categories) => Some(categories),
        Err(_) => None
    }
}