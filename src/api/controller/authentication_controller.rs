use std::error::Error;
use actix_web::{post, web, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use serde::Deserialize;
use validator::Validate;
use crate::errors::WrongCredentialsError::WrongCredentialsError;
use crate::service::security_service::authenticate;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(login);
}

#[post("/login")]
async fn login(payload: web::Json<LoginRequest>) -> impl Responder {
    let data = payload.into_inner();

    let token: Result<String, WrongCredentialsError> = authenticate(&data.username, &data.password);

    match token {
        Ok(token) => HttpResponse::Ok().body(token),
        Err(e) => HttpResponse::Forbidden().body(e.to_string())
    }
}

#[derive(Deserialize, Validate)]
struct LoginRequest {
    #[validate(length(min = 1, max = 50))]
    pub username: String,
    #[validate(length(min = 1, max = 50))]
    pub password: String,
}