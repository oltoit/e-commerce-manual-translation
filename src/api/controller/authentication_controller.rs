use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use crate::api::controller::connect::{get_connection, DbPool};
use crate::api::dto::auth_dto::{AuthResponse, LoginRequest};
use crate::service::security_service;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(login);
}

#[post("/login")]
async fn login(payload: web::Json<LoginRequest>, req: HttpRequest, pool: web::Data<DbPool>) -> impl Responder {
    let data = payload.into_inner();
    let path = req.match_info().as_str();
    let mut connection = match get_connection(pool, path) {
        Ok(conn) => conn,
        Err(response) => return response
    };

    match security_service::authenticate(&mut connection, data) {
        Ok(token) => HttpResponse::Ok().json(&AuthResponse { token }),
        Err(e) => e.get_response(path)
    }
}