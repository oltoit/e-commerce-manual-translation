use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use validator::Validate;
use crate::api::controller::connect::{get_connection, DbPool};
use crate::api::dto::auth_dto::{AuthResponse, LoginRequestDto};
use crate::service::security_service;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(login);
}

#[post("/login")]
async fn login(payload: web::Json<LoginRequestDto>, req: HttpRequest, pool: web::Data<DbPool>) -> impl Responder {
    let login_request = payload.into_inner();
    match login_request.validate() {
        Ok(_) => {},
        Err(_) => return HttpResponse::BadRequest().json("Invalid request")
    };
    let login_request = login_request.to_login_request();

    let path = req.match_info().as_str();
    let mut connection = match get_connection(pool, path) {
        Ok(conn) => conn,
        Err(response) => return response
    };

    match security_service::authenticate(&mut connection, login_request) {
        Ok(token) => HttpResponse::Ok().json(&AuthResponse { token }),
        Err(e) => e.get_response(path)
    }
}