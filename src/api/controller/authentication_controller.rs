use actix_web::{post, web, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::errors::error_enums::WrongCredentialsOrTokenParsingError;
use crate::errors::error_response_body::ErrorResponseBody;
use crate::service::security_service::authenticate;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(login);
}

#[post("/login")]
async fn login(payload: web::Json<LoginRequest>) -> impl Responder {
    let data = payload.into_inner();

    match authenticate(&data.username, &data.password) {
        // FIXME: remove unwrap
        Ok(token) => HttpResponse::Ok().body(serde_json::to_string(&AuthResponse { token }).unwrap()),
        Err(e) => match e {
            WrongCredentialsOrTokenParsingError::WrongCredentials => HttpResponse::Forbidden().body(forbidden("/login")),
            WrongCredentialsOrTokenParsingError::TokenParsing => HttpResponse::InternalServerError().body(internal_server_error("/login")),
        },
    }
}

#[derive(Deserialize, Validate)]
struct LoginRequest {
    #[validate(length(min = 1, max = 50))]
    pub username: String,
    #[validate(length(min = 1, max = 50))]
    pub password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    token: String,
}

fn internal_server_error(path: &str) -> String {
    serde_json::to_string(&ErrorResponseBody::internal_server_error(path.to_string())).unwrap()
}

fn forbidden(path: &str) -> String {
    serde_json::to_string(&ErrorResponseBody::forbidden(path.to_string())).unwrap()
}