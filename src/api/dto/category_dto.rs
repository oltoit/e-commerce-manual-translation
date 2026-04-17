use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateCategoryDto {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateCategoryDto {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
}