use diesel::QueryableByName;
use diesel::sql_types::BigInt;

#[derive(QueryableByName)]
pub struct Sequence {
    #[diesel(sql_type = BigInt)]
    pub id: i64,
}