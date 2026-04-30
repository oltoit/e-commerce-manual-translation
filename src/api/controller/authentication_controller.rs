use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use crate::api::controller::connect::{get_connection, DbPool};
use crate::api::dto::auth_dto::{AuthResponse, LoginRequestDto};
use crate::api::dto::validation_helper::validate_dto;
use crate::service::security_service;
use crate::api::error::ServiceError;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(login);
}

#[post("/login")]
async fn login<'a>(payload: web::Json<LoginRequestDto>, req: HttpRequest, pool: web::Data<DbPool>) -> Result<impl Responder, ServiceError> {
    let path = req.match_info().as_str();
    let login_request = payload.into_inner();
    let login_request = validate_dto(&login_request, path)?.to_login_request();
    let mut connection = get_connection(pool, path)?;

    Ok(HttpResponse::Ok().json(&AuthResponse {
        token: security_service::authenticate(&mut connection, login_request).map_err(|e| ServiceError::new(path.to_owned(), e))?
    }))
}