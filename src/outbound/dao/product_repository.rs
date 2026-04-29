use diesel::{PgConnection, QueryResult, RunQueryDsl, QueryDsl};
use diesel::prelude::*;
use diesel::query_dsl::methods::OffsetDsl;
use crate::api::controller::pagination::Pagination;
use crate::outbound::dao::sequence_repository::get_next_val;
use crate::shared::entity::category::Category;
use crate::shared::entity::user::User;
use crate::shared::entity::product::{NewProduct, Product, ProductSort, ProductWithUser, UpdateProduct};
use crate::schema::app_category;
use crate::schema::app_product;
use crate::schema::app_product::id;
use crate::schema::app_product_category::{productid};
use crate::schema::app_product_category::dsl::app_product_category;
use crate::schema::app_user;

pub fn get_all_products(connection: &mut PgConnection, pagination: &Pagination) -> QueryResult<Vec<Product>> {
    use crate::schema::app_product::dsl::app_product;

    let sorts = match ProductSort::from_str_vec(pagination.get_unsanitized_sorts()) {
        Ok(sorts) => sorts,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    let mut order = app_product.into_boxed();
    for sort in sorts {
        order = sort.apply_to_query(order);
    }

    OffsetDsl::offset(order
        .limit(pagination.get_page()), pagination.get_page() * pagination.get_size())
        .get_results(connection)
}

pub fn get_all_products_with_user(connection: &mut PgConnection, pagination: &Pagination) -> QueryResult<(Vec<ProductWithUser>, i64)> {
    use crate::schema::app_product::dsl::app_product;
    let total = app_product.count().get_result::<i64>(connection)?;

    let sorts = match ProductSort::from_str_vec(pagination.get_unsanitized_sorts()) {
        Ok(sorts) => sorts,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    let mut order = app_product.into_boxed();
    for sort in sorts {
        order = sort.apply_to_query(order);
    }

    let products_with_users = OffsetDsl::offset(order
        .limit(pagination.get_size()), pagination.get_size() * pagination.get_page())
        .inner_join(app_user::table)
        .select((Product::as_select(), User::as_select()))
        .get_results::<(Product, User)>(connection)?
        .into_iter()
        .map(|(product, user)| ProductWithUser::from(product, user))
        .collect();

    Ok((products_with_users, total))
}

pub fn get_by_id(connection: &mut PgConnection, product_id: i64) -> QueryResult<Product> {
    use crate::schema::app_product::dsl::app_product;
    app_product.find(product_id).select(Product::as_select()).get_result(connection)
}

pub fn get_by_id_with_user(connection: &mut PgConnection, product_id: i64) -> QueryResult<ProductWithUser> {
    use crate::schema::app_product::dsl::app_product;

    let (product, user) = app_product
        .inner_join(app_user::table)
        .filter(id.eq(product_id))
        .select((Product::as_select(), User::as_select()))
        .get_result(connection)?;

    Ok(ProductWithUser::from(product, user))
}

pub fn insert(connection: &mut PgConnection, mut new_product: NewProduct) -> QueryResult<Product> {
    let next_id = get_next_val(connection)?;

    new_product.set_id(next_id.id);

    diesel::insert_into(app_product::table)
        .values(new_product)
        .returning(Product::as_returning())
        .get_result(connection)
}

pub fn insert_return_with_user(connection: &mut PgConnection, mut new_product: NewProduct) -> QueryResult<ProductWithUser> {
    use crate::schema::app_user::dsl::app_user;

    let next_id = get_next_val(connection)?;

    new_product.set_id(next_id.id);

    let created_product = diesel::insert_into(app_product::table)
        .values(new_product)
        .returning(Product::as_returning())
        .get_result(connection)?;

    let user = app_user.find(created_product.userid).get_result(connection)?;

    Ok(ProductWithUser::from(created_product, user))
}

pub fn update(connection: &mut PgConnection, update_product: UpdateProduct, product_id: i64) -> QueryResult<Product> {
    use crate::schema::app_product::dsl::app_product;

    diesel::update(app_product.find(product_id)).set(update_product).get_result(connection)
}

pub fn update_return_with_user(connection: &mut PgConnection, update_product: UpdateProduct, product_id: i64) -> QueryResult<ProductWithUser> {
    use crate::schema::app_product::dsl::app_product;
    use crate::schema::app_user::dsl::app_user;

    let updated_product = diesel::update(app_product.find(product_id)).set(update_product).get_result::<Product>(connection)?;
    let user = app_user.find(updated_product.userid).get_result(connection)?;

    Ok(ProductWithUser::from(updated_product, user))
}

pub fn delete(connection: &mut PgConnection, product_id: i64) -> QueryResult<usize> {
    use crate::schema::app_product::dsl::app_product;
    
    diesel::delete(app_product.find(product_id)).execute(connection)
}

pub fn get_categories_for_product(connection: &mut PgConnection, product_id: i64) -> QueryResult<Vec<Category>> {
    app_product_category
        .filter(productid.eq(product_id))
        .inner_join(app_category::table)
        .select(Category::as_select())
        .get_results(connection)
}