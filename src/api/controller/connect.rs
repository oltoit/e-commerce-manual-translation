use diesel::{Connection, PgConnection};
use crate::config::env_loader::LOADER;

pub fn connect() -> PgConnection {
    // FIXME: remove unwrap
    let database_url = LOADER.get().unwrap().get_database_url();
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}