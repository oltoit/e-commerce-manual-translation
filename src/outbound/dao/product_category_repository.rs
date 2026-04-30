use diesel::prelude::*;
use diesel::{sql_query, PgConnection, QueryDsl, QueryResult, RunQueryDsl, SelectableHelper};
use diesel::result::Error;
use diesel::sql_types::BigInt;
use crate::api::controller::pagination_helper::Pagination;
use crate::shared::entity::product::{product_sorts_to_sql_string, Product, ProductSort, ProductWithUser};
use crate::shared::entity::product_category_relation::ProductCategoryRelation;
use crate::shared::entity::user::User;
use crate::schema::app_product_category;
use crate::schema::app_product::dsl::app_product;
use crate::schema::app_product_category::{categoryid, productid};
use crate::schema::app_user::dsl::app_user;
use crate::schema::app_user::id;

/// Query taken directly from source-application
pub fn get_all_products_for_categories_recursive(connection: &mut PgConnection, pagination: &Pagination, category_id: i64) -> QueryResult<(Vec<ProductWithUser>, i64)> {
    let limit = pagination.get_size();
    let offset = pagination.get_page() * limit;
    let sort = match ProductSort::from_str_vec_product_category(pagination.get_unsanitized_sorts()) {
        Ok(sort) => sort,
        Err(_) => return Err(Error::NotFound)
    };
    let sort = product_sorts_to_sql_string(sort);

    let products: Vec<Product> = sql_query(format!("\
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

    let users = app_user.filter(id.eq_any(products.iter().map(|p| p.userid).collect::<Vec<i64>>())).load::<User>(connection)?;
    let result = products.into_iter().map(|p| {
        let user = users.iter().find(|u| u.id == p.userid).ok_or(Error::NotFound)?.clone();
        Ok(ProductWithUser::from(p, user))
    }).collect::<Result<Vec<ProductWithUser>, Error>>()?;

    let total = app_product.count().get_result::<i64>(connection)?;

    Ok((result, total))
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