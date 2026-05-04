use actix_web::web;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use crate::shared::env_loader::get_loader;
use crate::api::error::ServiceError;
use crate::shared::errors::error_enum::ErrorsEnum;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConn = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_connection(pool: web::Data<DbPool>, path: &str) -> Result<DbConn, ServiceError> {
    match pool.get() {
        Ok(conn) => Ok(conn),
        Err(_) => Err(ServiceError::new(path.to_string(), ErrorsEnum::DatabaseError("Could not get connection from pool".to_string())))
    }
}

pub fn create_pool() -> Result<DbPool, ErrorsEnum> {
    let loader = get_loader()?;
    let database_url = loader.get_database_url();
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .max_size(loader.get_connection_pool_max_size())
        .min_idle(loader.get_connection_pool_min_idle())
        .build(manager)
        .map_err(|_| ErrorsEnum::DatabaseError("Could not create pool".to_string()))
}