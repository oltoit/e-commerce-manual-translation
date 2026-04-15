use diesel::{PgConnection, QueryResult, RunQueryDsl};
use crate::entity::sequence::Sequence;

pub fn get_next_val(connection: &mut PgConnection) -> QueryResult<Sequence> {
    diesel::sql_query("SELECT nextval('hibernate_sequence') AS id")
        .get_result::<Sequence>(connection)
}