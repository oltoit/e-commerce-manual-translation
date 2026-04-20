use actix_web::{options, post, web, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use crate::api::controller::connect::connect;
use crate::api::dto::auth_dto::{AuthResponse, LoginRequest};
use crate::service::security_service;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(options_login);
    cfg.service(login);
}

#[options("/login")]
async fn options_login() -> impl Responder { HttpResponse::Ok().finish() }

#[post("/login")]
async fn login(payload: web::Json<LoginRequest>) -> impl Responder {
    let data = payload.into_inner();
    let mut connection = connect();

    match security_service::authenticate(&mut connection, data) {
        Ok(token) => HttpResponse::Ok().json(&AuthResponse { token }),
        Err(e) => e.get_response("/login")
    }
}