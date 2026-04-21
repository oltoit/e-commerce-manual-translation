use diesel::prelude::*;
use diesel::{sql_query, PgConnection, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};
use diesel::sql_types::BigInt;
use crate::api::controller::pagination::Pagination;
use crate::entity::category::Category;
use crate::entity::product::{product_sorts_to_sql_string, Product, ProductSort};
use crate::entity::product_category_relation::ProductCategoryRelation;
use crate::schema::{app_category, app_product_category};
use crate::schema::app_product::dsl::app_product;
use crate::schema::app_product_category::{categoryid, productid};

// TODO: refactor this query to be simpler!!!
// TODO: test if this actually works
/// Query taken directly from source-application
pub fn get_all_products_for_categories_recursive(connection: &mut PgConnection, pagination: &Pagination, category_id: i64) -> QueryResult<(Vec<Product>, i64)> {
    let limit = pagination.get_size();
    let offset = pagination.get_page() * limit;
    let sort = match ProductSort::from_str_vec(pagination.get_unsanitized_sorts()) {
        Ok(sort) => sort,
        Err(_) => return Err(diesel::result::Error::NotFound)
    };
    let sort = product_sorts_to_sql_string(sort);

    let result = sql_query(format!("\
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
        )\
        ORDER BY {} OFFSET $2 LIMIT $3", sort)
    )
    .bind::<BigInt, _>(category_id)
    .bind::<BigInt, _>(offset)
    .bind::<BigInt, _>(limit)
    .load(connection)?;

    let total = app_product.count().get_result::<i64>(connection)?;

    Ok((result, total))
}

pub fn get_all_categories_for_product(connection: &mut PgConnection, product_id: i64) -> QueryResult<Vec<Category>> {
    use crate::schema::app_product_category::dsl::app_product_category;

    app_product_category
        .filter(productid.eq(product_id))
        .inner_join(app_category::table)
        .select(Category::as_select())
        .get_results(connection)
}

/// returns error if product is not associated with category
pub fn get_product_for_category(connection: &mut PgConnection, category_id: i64, product_id: i64) -> QueryResult<ProductCategoryRelation> {
    use crate::schema::app_product_category::dsl::app_product_category;

    app_product_category.filter(categoryid.eq(category_id).and(productid.eq(product_id))).first(connection)
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