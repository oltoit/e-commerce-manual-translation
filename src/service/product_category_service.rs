use diesel::PgConnection;
use crate::dao::product_category_repository::{get_all_products_for_categories_recursive, get_all_products_for_category};
use crate::entity::product::Product;

// FIXME: should fail if category does not exist
pub fn get_products_for_category(connection: &mut PgConnection, category_id: i64) -> Vec<Product> {
    let result = get_all_products_for_category(connection, category_id);

    result.unwrap_or(vec![])
}

// FIXME: database throws an error -> query is probably wrong
pub fn get_products_for_categories_recursive(connection: &mut PgConnection, category_id: i64) -> Vec<Product> {
    let result = get_all_products_for_categories_recursive(connection, category_id);

    result.unwrap_or(vec![])
}