use diesel::{PgConnection, QueryResult, RunQueryDsl, QueryDsl};
use diesel::prelude::*;
use crate::dao::sequence_repository::get_next_val;
use crate::entity::category::Category;
use crate::entity::product::{NewProduct, Product, UpdateProduct};
use crate::schema::app_category;
use crate::schema::app_product;
use crate::schema::app_product_category::{productid};
use crate::schema::app_product_category::dsl::app_product_category;

pub fn get_all_products(connection: &mut PgConnection) -> QueryResult<Vec<Product>> {
    use crate::schema::app_product::dsl::app_product;
    app_product.select(Product::as_select()).load(connection)
}

// TODO: think wbout how to do this in regards with Option and so on -> probably have Service return Option
pub fn get_by_id(connection: &mut PgConnection, id: i64) -> QueryResult<Product> {
    use crate::schema::app_product::dsl::app_product;
    app_product.find(id).select(Product::as_select()).get_result(connection)
}


pub fn insert(connection: &mut PgConnection, mut new_product: NewProduct) -> QueryResult<Product> {
    let next_id = get_next_val(connection)?;

    new_product.set_id(next_id.id);

    diesel::insert_into(app_product::table)
        .values(new_product)
        .returning(Product::as_returning())
        .get_result(connection)
}

pub fn update(connection: &mut PgConnection, update_product: UpdateProduct, id: i64) -> QueryResult<Product> {
    use crate::schema::app_product::dsl::app_product;

    diesel::update(app_product.find(id)).set(update_product).get_result(connection)
}

pub fn delete(connection: &mut PgConnection, id: i64) -> QueryResult<usize> {
    use crate::schema::app_product::dsl::app_product;
    
    diesel::delete(app_product.find(id)).execute(connection)
}

// TODO: see if this actually works
pub fn get_categories_for_product(connection: &mut PgConnection, product_id: i64) -> QueryResult<Vec<Category>> {
    app_product_category
        .filter(productid.eq(product_id))
        .inner_join(app_category::table)
        .select(Category::as_select())
        .get_results(connection)
}