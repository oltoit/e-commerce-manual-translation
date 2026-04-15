use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, QueryableByName, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::app_category)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category{
    pub id: i64,
    pub name: String,
    pub parentid: Option<i64>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::app_category)]
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
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::app_category)]
pub struct UpdateCategory<'a> {
    pub name: &'a str,
    pub parentid: Option<i64>,
}