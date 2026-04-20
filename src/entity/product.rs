use diesel::prelude::*;

#[derive(Queryable, QueryableByName, Selectable)]
#[diesel(table_name = crate::schema::app_product)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub userid: i64,
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