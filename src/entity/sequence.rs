use diesel::QueryableByName;
use diesel::sql_types::BigInt;
use diesel::prelude::*;

#[derive(QueryableByName)]
pub struct Sequence {
    #[sql_type = "BigInt"]
    pub id: i64,
}