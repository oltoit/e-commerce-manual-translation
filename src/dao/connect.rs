use std::env;
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;

pub fn connect() -> PgConnection {
    dotenv().ok();

    // FIXME: no expect, for now I'll let it slide though
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}