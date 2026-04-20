use diesel::pg::Pg;
use diesel::prelude::*;
use crate::api::dto::product_dto::{CreateProductDto, UpdateProductDto};
use crate::entity::user::User;
use crate::errors::error_enum::ErrorsEnum;
use crate::schema::app_product::BoxedQuery;
use crate::security::auth_context_holder::AuthUser;

#[derive(Queryable, QueryableByName, Selectable, Identifiable, Associations)]
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
    pub fn from_dto(dto: &'a CreateProductDto, auth_user: &AuthUser) -> Self {
        NewProduct {id: None, name: dto.name.as_str(), price: dto.price, userid: auth_user.id}
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
    pub fn from_dto(dto: &'a UpdateProductDto, auth_user: &AuthUser) -> Self {
        UpdateProduct {name: dto.name.as_str(), price: dto.price, userid: auth_user.id}
    }
}

/* Sort options for products */

pub struct ProductSort {
    pub field: String,
    pub order: String,
}

pub const DEFAULT_SORT_ORDER: &str = "asc";
pub const DEFAULT_SORT_FIELD: &str = "id";
const SORT_ORDER_OPTIONS: [&str; 2] = ["asc", "desc"];
const SORT_FIELD_OPTIONS: [&str; 4] = ["id", "name", "price", "user"];

impl ProductSort {
    pub fn apply_to_query<'a>(&self, query: BoxedQuery<'a, Pg>) -> BoxedQuery<'a, Pg> {
        match (self.field.as_str(), self.field.as_str()) {
            ("name", "desc") => query.order(crate::schema::app_product::name.desc()),
            ("name", "asc") => query.order(crate::schema::app_product::name.asc()),
            ("price", "desc") => query.order(crate::schema::app_product::price.desc()),
            ("price", "asc") => query.order(crate::schema::app_product::price.asc()),
            ("user", "desc") => query.order(crate::schema::app_product::userid.desc()),
            ("user", "asc") => query.order(crate::schema::app_product::userid.asc()),
            (_, "desc") => query.order(crate::schema::app_product::id.desc()),
            (_, _) => query.order(crate::schema::app_product::id.asc()),
        }
    }

    pub fn from_str(sort: &str) -> Result<Self, ErrorsEnum> {
        let mut split = sort.split(',');
        let field = split.next().unwrap_or(DEFAULT_SORT_FIELD).to_string();
        let order = split.next().unwrap_or(DEFAULT_SORT_ORDER).to_string();

        let field = validate_field(&field)?;
        let order = validate_order(&order)?;

        Ok(ProductSort { field, order })
    }
}

fn validate_field(field: &str) -> Result<String, ErrorsEnum> {
    if SORT_FIELD_OPTIONS.contains(&field) {
        // the source-application uses user instead of userid for sorting, so user needs to be mapped to userid here
        match field {
            "user" => Ok("userid".to_string()),
            _ => Ok(field.to_string()),
        }
    } else {
        Err(ErrorsEnum::NoPropertyError(format!("no property '{}' found", field)))
    }
}
fn validate_order(order: &str) -> Result<String, ErrorsEnum> {
    if SORT_ORDER_OPTIONS.contains(&order) {
        Ok(order.to_string())
    } else {
        Err(ErrorsEnum::NoPropertyError(format!("no order '{}' found", order)))
    }
}