use actix_web::{get, options, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use crate::api::resource::category_resource::CategoryResource;
use crate::dao::connect::connect;
use crate::errors::error_response_body::ErrorResponseBody;
use crate::service::category_service;

// TODO: register all routes from controller here
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_categories);
}

#[options("/categories")]
async fn options_categories() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("/categories")]
async fn get_categories() -> impl Responder {
    let mut connection = connect();

    let result = match category_service::get_categories(&mut connection) {
        Ok(categories) => categories,
        Err(_) => return HttpResponse::InternalServerError().body(internal_server_error("/categories".to_string()))
    };

    let resources = result.iter().map(|r| CategoryResource::from_entity(&mut connection, r)).collect::<Vec<CategoryResource>>();

    HttpResponse::Ok().body(serde_json::to_string(&resources).unwrap())
}

fn internal_server_error(path: String) -> String {
    serde_json::to_string(&ErrorResponseBody::internal_server_error(path)).unwrap()
}