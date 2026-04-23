use diesel::{Connection, PgConnection};
use crate::config::env_loader::get_loader;
use crate::errors::error_enum::ErrorsEnum;

pub fn connect() -> Result<PgConnection, ErrorsEnum> {
    let database_url = get_loader()?.get_database_url();
    PgConnection::establish(&database_url).map_err(|_| ErrorsEnum::DatabaseError("Could not connect to database".to_string()))
}