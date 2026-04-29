use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};
use crate::outbound::dao::sequence_repository::get_next_val;
use crate::shared::entity::category::{Category, NewCategory, UpdateCategory};
use crate::schema::app_category;
use crate::schema::app_category::parentid;

pub fn get_all_categories(connection: &mut PgConnection) -> QueryResult<Vec<Category>> {
    use crate::schema::app_category::dsl::app_category;

    app_category.select(Category::as_select()).get_results(connection)
}

pub fn get_by_id(connection: &mut PgConnection, category_id: i64) -> QueryResult<Category> {
    use crate::schema::app_category::dsl::app_category;

    app_category.find(category_id).select(Category::as_select()).get_result(connection)
}

pub fn insert(connection: &mut PgConnection, mut new_category: NewCategory) -> QueryResult<Category> {
    let next_id = get_next_val(connection)?;

    new_category.set_id(next_id.id);

    diesel::insert_into(app_category::table)
        .values(new_category)
        .returning(Category::as_returning())
        .get_result(connection)
}

pub fn update(connection: &mut PgConnection, update_category: UpdateCategory, category_id: i64) -> QueryResult<Category> {
    use crate::schema::app_category::dsl::app_category;

    diesel::update(app_category.find(category_id)).set(update_category).get_result(connection)
}

pub fn delete(connection: &mut PgConnection, category_id: i64) -> QueryResult<usize> {
    use crate::schema::app_category::dsl::app_category;

    diesel::delete(app_category.find(category_id)).execute(connection)
}

/* Subcategory functions */

pub fn get_subcategories(connection: &mut PgConnection, category_id: i64) -> QueryResult<Vec<Category>> {
    use crate::schema::app_category::dsl::app_category;

    app_category.find(category_id).get_result::<Category>(connection)?;
    app_category.filter(parentid.eq(category_id)).select(Category::as_select()).get_results(connection)
}