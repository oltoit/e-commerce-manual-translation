use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = crate::schema::app_product_category)]
#[diesel(primary_key(productid, categoryid))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProductCategoryRelation {
    pub productid: i64,
    pub categoryid: i64,
}