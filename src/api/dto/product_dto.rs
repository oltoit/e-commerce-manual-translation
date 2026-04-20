use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateProductDto {
    #[validate(length(min = 1, max = 300))]
    pub name: String,
    #[validate(length(min = 3, max = 3))]
    pub currency: String,
    #[validate(range(min = 0.0))]
    pub price: f64,
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