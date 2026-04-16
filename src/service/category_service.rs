use diesel::PgConnection;
use crate::dao::category_repository::{get_all_categories, get_subcategories};
use crate::entity::category::Category;

// TODO: authorization -> see how auth error could be passed to controller
// TODO: manually make every service method a transaction
pub fn get_categories(connection: &mut PgConnection) -> Result<Vec<Category>, diesel::result::Error> {
    let result = get_all_categories(connection);

    match result {
        Ok(categories) => Ok(categories),
        Err(e) => Err(e)
    }
}

pub fn get_subcategories_for_id(connection: &mut PgConnection, id: i64) -> Option<Vec<Category>> {
    let result = get_subcategories(connection, id);

    match result {
        Ok(categories) => Some(categories),
        Err(_) => None
    }
}