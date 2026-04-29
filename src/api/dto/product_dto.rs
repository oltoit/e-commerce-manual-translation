use serde::Deserialize;
use validator::Validate;
use crate::shared::entity::product::{NewProduct, UpdateProduct};

#[derive(Deserialize, Validate)]
pub struct CreateProductDto {
    #[validate(length(min = 1, max = 300))]
    pub name: String,
    #[validate(length(min = 3, max = 3))]
    pub currency: String,
    #[validate(range(min = 0.0))]
    pub price: f64,
}

impl CreateProductDto {
    pub fn to_new_product(&self, user_id: i64) -> NewProduct<'_> {
        NewProduct::new(&self.name, self.price, user_id)
    }
}

#[derive(Deserialize, Validate)]
pub struct UpdateProductDto {
    #[validate(length(min = 1, max = 300))]
    pub name: String,
    #[validate(length(min = 3, max = 3))]
    pub currency: String,
    #[validate(range(min = 0.0))]
    pub price: f64,
}

impl UpdateProductDto {
    pub fn to_update_product(&self, user_id: i64) -> UpdateProduct<'_> {
        UpdateProduct::new(&self.name, self.price, user_id)
    }
}