use serde::Deserialize;
use validator::Validate;
use crate::shared::entity::category::{NewCategory, UpdateCategory};

#[derive(Deserialize, Validate)]
pub struct CreateCategoryDto {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
}

impl CreateCategoryDto {
    pub fn to_new_category(&self) -> NewCategory<'_> {
        NewCategory::new(&self.name, None)
    }
}

#[derive(Deserialize, Validate)]
pub struct UpdateCategoryDto {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
}

impl UpdateCategoryDto {
    pub fn to_update_category(&self) -> UpdateCategory<'_> {
        UpdateCategory::new(&self.name, None)
    }
}