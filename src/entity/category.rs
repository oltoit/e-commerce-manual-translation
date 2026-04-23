use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::api::dto::category_dto::{CreateCategoryDto, UpdateCategoryDto};

#[derive(Queryable, QueryableByName, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::app_category)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category{
    pub id: i64,
    pub name: String,
    pub parentid: Option<i64>,
}

#[derive(QueryableByName)]
#[diesel(table_name = crate::schema::app_category)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CategoryId{
    pub id: i64,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::app_category)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewCategory<'a> {
    id: Option<i64>,
    pub name: &'a str,
    pub parentid: Option<i64>,
}

impl<'a> NewCategory<'a> {
    pub fn new(name: &'a str, parentid: Option<i64>) -> Self {
        NewCategory {id: None, name, parentid}
    }
    pub fn set_id(&mut self, new_id: i64) {
        self.id = Some(new_id);
    }
    pub fn from(create_category: &'a CreateCategoryDto) -> Self {
        NewCategory {id: None, name: &create_category.name, parentid: None}
    }
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::app_category)]
#[diesel(treat_none_as_null = true)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateCategory<'a> {
    pub name: &'a str,
    pub parentid: Option<i64>,
}

impl<'a> UpdateCategory<'a> {
    pub fn from(update_category: &'a UpdateCategoryDto) -> Self {
        UpdateCategory {name: &update_category.name, parentid: None}
    }

    pub fn from_category(category: &'a Category) -> Self {
        UpdateCategory {name: &category.name, parentid: category.parentid}
    }
}