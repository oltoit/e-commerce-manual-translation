use actix_web::{post, web, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use crate::api::dto::auth_dto::{AuthResponse, LoginRequest};
use crate::service::security_service::authenticate;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(login);
}

#[post("/login")]
async fn login(payload: web::Json<LoginRequest>) -> impl Responder {
    let data = payload.into_inner();

    match authenticate(&data.username, &data.password) {
        Ok(token) => HttpResponse::Ok().json(&AuthResponse { token }),
        Err(e) => e.get_response("/login".to_string())
    }
}