use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_dsl::methods::ThenOrderDsl;
use crate::shared::entity::user::User;
use crate::shared::errors::error_enum::ErrorsEnum;
use crate::schema::app_product::BoxedQuery;

#[derive(Queryable, QueryableByName, Selectable, Identifiable, Associations, Clone)]
#[diesel(table_name = crate::schema::app_product)]
#[diesel(belongs_to(User, foreign_key = userid))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub userid: i64,
}

pub struct ProductWithUser {
    pub product: Product,
    pub user: User,
}
impl ProductWithUser {
    pub fn from(product: Product, user: User) -> Self {
        Self { product, user }
    }
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::app_product)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewProduct<'a> {
    id: Option<i64>,
    pub name: &'a str,
    pub price: f64,
    pub userid: i64,
}

impl<'a> NewProduct<'a> {
    pub fn new(name: &'a str, price: f64, userid: i64) -> Self {
        NewProduct {id: None, name, price, userid}
    }
    pub fn set_id(&mut self, new_id: i64) {
        self.id = Some(new_id);
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::app_product)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateProduct<'a> {
    pub name: &'a str,
    pub price: f64,
    pub userid: i64,
}
impl<'a> UpdateProduct<'a> {
    pub fn new(name: &'a str, price: f64, userid: i64) -> Self {
        Self { name, price, userid }
    }
}

/* Sort options for products */

pub struct ProductSort {
    field: FieldOptions,
    order: OrderOptions,
}

pub const DEFAULT_SORT_ORDER: &str = "asc";
pub const DEFAULT_SORT_FIELD: &str = "id";

enum OrderOptions { Asc, Desc }
impl OrderOptions {
    fn from_str(order: &str) -> Option<Self> {
        match order.to_ascii_lowercase().as_str() {
            "asc" => Some(OrderOptions::Asc),
            "desc" => Some(OrderOptions::Desc),
            _ => None,
        }
    }
    fn to_string(&self) -> String {
        match self {
            OrderOptions::Asc => "asc".to_string(),
            OrderOptions::Desc => "desc".to_string(),
        }
    }
}
enum FieldOptions { Id, Name, Price, User }
impl FieldOptions {
    fn from_str(order: &str) -> Option<Self> {
        match order.to_ascii_lowercase().as_str() {
            "id" => Some(FieldOptions::Id),
            "name" => Some(FieldOptions::Name),
            "price" => Some(FieldOptions::Price),
            "user" => Some(FieldOptions::User),
            _ => None,
        }
    }
    fn to_sql_fields(&self) -> String {
        match self {
            FieldOptions::Id => "id".to_string(),
            FieldOptions::Name => "name".to_string(),
            FieldOptions::Price => "price".to_string(),
            FieldOptions::User => "userid".to_string(),
        }
    }
}

impl ProductSort {
    pub fn apply_to_query<'a>(&self, query: BoxedQuery<'a, Pg>) -> BoxedQuery<'a, Pg> {
        match (&self.field, &self.order) {
            (FieldOptions::Id, OrderOptions::Asc) => ThenOrderDsl::then_order_by(query, crate::schema::app_product::id.asc()),
            (FieldOptions::Id, OrderOptions::Desc) => ThenOrderDsl::then_order_by(query, crate::schema::app_product::id.desc()),
            (FieldOptions::Name, OrderOptions::Asc) => ThenOrderDsl::then_order_by(query, crate::schema::app_product::name.asc()),
            (FieldOptions::Name, OrderOptions::Desc) => ThenOrderDsl::then_order_by(query, crate::schema::app_product::name.desc()),
            (FieldOptions::Price, OrderOptions::Asc) => ThenOrderDsl::then_order_by(query, crate::schema::app_product::price.asc()),
            (FieldOptions::Price, OrderOptions::Desc) => ThenOrderDsl::then_order_by(query, crate::schema::app_product::price.desc()),
            (FieldOptions::User, OrderOptions::Asc) => ThenOrderDsl::then_order_by(query, crate::schema::app_product::userid.asc()),
            (FieldOptions::User, OrderOptions::Desc) => ThenOrderDsl::then_order_by(query, crate::schema::app_product::userid.desc()),
        }
    }

    /// In product-controller 'user' is the valid way to sort for user.
    /// This is the default option and should be chosen in most scenarios.
    pub fn from_str_vec(sorts: Vec<&str>) -> Result<Vec<Self>, ErrorsEnum> {
        Self::from_str_internal(sorts, validate_field_where_user_valid)
    }

    /// In category-products-controller 'userid' is the valid way to sort for user.
    pub fn from_str_vec_product_category(sorts: Vec<&str>) -> Result<Vec<Self>, ErrorsEnum> {
        Self::from_str_internal(sorts, validate_field_where_userid_valid)
    }

    fn from_str_internal(sorts: Vec<&str>, validate_function: fn(&str) -> Result<FieldOptions, ErrorsEnum>) -> Result<Vec<Self>, ErrorsEnum> {
        let mut product_sorts = Vec::with_capacity(sorts.len());

        for sort in sorts {
            let mut split = sort.split(',');
            let field = split.next().unwrap_or(DEFAULT_SORT_FIELD).to_string();
            let order = split.next().unwrap_or(DEFAULT_SORT_ORDER).to_string();

            let field = validate_function(&field)?;
            let order = validate_order(&order)?;

            product_sorts.push(ProductSort { field, order });
        }

        Ok(product_sorts)
    }
}

pub fn product_sorts_to_sql_string(sorts: Vec<ProductSort>) -> String {
    let mut sql_string = String::new();

    for sort in sorts {
        sql_string.push_str(&format!("{} {}, ", sort.field.to_sql_fields(), sort.order.to_string()));
    }

    // trim last comma since sql will fail if it's still there
    if let Some(i) = sql_string.rfind(',') {
        sql_string.truncate(i);
    }

    sql_string
}

fn validate_field_where_user_valid(field: &str) -> Result<FieldOptions, ErrorsEnum> {
    match FieldOptions::from_str(field) {
        Some(field) => Ok(field),
        None => Err(ErrorsEnum::NoPropertyError(format!("no field '{}' found", field))),
    }
}

fn validate_field_where_userid_valid(field: &str) -> Result<FieldOptions, ErrorsEnum> {
    let field = match field {
        "userid" => "user",
        "user" => return Err(ErrorsEnum::NoPropertyError(format!("no field '{}' found", field))),
        _ => field,
    };

    match FieldOptions::from_str(field) {
        Some(field) => Ok(field),
        None => Err(ErrorsEnum::NoPropertyError(format!("no field '{}' found", field))),
    }
}

fn validate_order(order: &str) -> Result<OrderOptions, ErrorsEnum> {
    match OrderOptions::from_str(order) {
        Some(order) => Ok(order),
        None => Err(ErrorsEnum::NoPropertyError(format!("no order '{}' found", order))),
    }
}