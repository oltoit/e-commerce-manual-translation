use diesel::prelude::*;
use diesel::{sql_query, PgConnection, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};
use diesel::sql_types::BigInt;
use crate::entity::category::Category;
use crate::entity::product::Product;
use crate::entity::product_category_relation::ProductCategoryRelation;
use crate::schema::{app_category, app_product, app_product_category};
use crate::schema::app_product_category::{categoryid, productid};

pub fn get_all_products_for_category(connection: &mut PgConnection, category_id: i64) -> QueryResult<Vec<Product>> {
    use crate::schema::app_product_category::dsl::app_product_category;

    app_product_category
        .filter(categoryid.eq(category_id))
        .inner_join(app_product::table)
        .select(Product::as_select())
        .get_results(connection)
}

// TODO: refactor this query to be simpler!!!
/// Query taken directly from source-application
pub fn get_all_products_for_categories_recursive(connection: &mut PgConnection, category_id: i64) -> QueryResult<Vec<Product>> {
    sql_query("\
        select p.* from app_product p \
        inner join app_product_category pc on p.id = pc.productid \
        where (\
            pc.categoryid = $1 or \
            pc.categoryid in (\
                select ac.id from (\
                    WITH RECURSIVE ALL_SUBCATEGORIES(ID, PARENTID) \
                    AS (select c.id, c.parentid from app_category c where c.parentid is null union all select c.id, c.parentid \
                    FROM ALL_SUBCATEGORIES inner join app_category c on ALL_SUBCATEGORIES.id = c.parentid) \
                    select id, parentid from ALL_SUBCATEGORIES \
                ) ac where ac.parentid = $1\
            )\
        )"
    )
    .bind::<BigInt, _>(category_id)
    .bind::<BigInt, _>(category_id)
    .load(connection)
}

pub fn get_all_categories_for_product(connection: &mut PgConnection, product_id: i64) -> QueryResult<Vec<Category>> {
    use crate::schema::app_product_category::dsl::app_product_category;

    app_product_category
        .filter(productid.eq(product_id))
        .inner_join(app_category::table)
        .select(Category::as_select())
        .get_results(connection)
}

pub fn add_category_to_product(connection: &mut PgConnection, relation: ProductCategoryRelation) -> QueryResult<ProductCategoryRelation> {
    diesel::insert_into(app_product_category::table)
        .values(relation)
        .returning(ProductCategoryRelation::as_returning())
        .get_result(connection)
}

pub fn remove_category_from_product(connection: &mut PgConnection, relation: ProductCategoryRelation) -> QueryResult<usize> {
    diesel::delete(app_product_category::table.find((relation.productid, relation.categoryid))).execute(connection)
}