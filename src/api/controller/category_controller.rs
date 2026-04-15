use actix_web::{get, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use crate::api::resource::category_resource::CategoryResource;
use crate::dao::connect::connect;
use crate::service::category_service;

// TODO: register all routes from controller here
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_categories);
}

#[get("/categories")]
async fn get_categories() -> impl Responder {
    let mut connection = connect();

    let result = match category_service::get_categories(&mut connection) {
        Ok(categories) => categories,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string())
    };

    let resources = result.iter().map(|r| CategoryResource::from_entity(&mut connection, r)).collect::<Vec<CategoryResource>>();

    HttpResponse::Ok().body(serde_json::to_string(&resources).unwrap())
}