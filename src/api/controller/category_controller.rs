use actix_web::{get, options, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use crate::api::resource::category_resource::{CategoryResource, CategoryResourceHal};
use crate::dao::connect::connect;
use crate::errors::error_response_body::ErrorResponseBody;
use crate::security::auth_context_holder::AuthUser;
use crate::service::category_service;
use crate::service::security_service::authenticate;

// TODO: register all routes from controller here
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(options_categories);
    cfg.service(get_categories);
    cfg.service(get_category);
}

#[options("/categories")]
async fn options_categories() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("/categories")]
async fn get_categories(req: HttpRequest) -> impl Responder {
    // TODO: remove unwrap
    // FIXME: add auth check to every service function
    let auth_user = req.extensions().get::<AuthUser>().unwrap();
    let mut connection = connect();

    let result = match category_service::get_categories(&mut connection, auth_user) {
        Some(categories) => categories,
        None => return HttpResponse::InternalServerError().body(internal_server_error("/categories".to_string()))
    };

    let resources = result.iter().map(|r| CategoryResource::from_entity(&mut connection, r)).collect::<Vec<CategoryResource>>();
    // TODO: remove unwrap
    HttpResponse::Ok().body(serde_json::to_string(&resources).unwrap())
}

#[get("/categories/{id}")]
async fn get_category(path: web::Path<i64>) -> impl Responder {
    let mut connection = connect();
    let id = path.into_inner();

    let result = match category_service::get_category_by_id(&mut connection, id) {
        Some(category) => category,
        None => return HttpResponse::NotFound().body(not_found(format!("/categories/{}", id)))
    };

    let resource = CategoryResourceHal::from_entity(&mut connection, &result);
    // TODO: remove unwrap
    HttpResponse::Ok().body(serde_json::to_string(&resource).unwrap())
}



fn internal_server_error(path: String) -> String {
    // TODO: remove unwrap
    serde_json::to_string(&ErrorResponseBody::internal_server_error(path)).unwrap()
}
fn not_found(path: String) -> String {
    // TODO: remove unwrap
    serde_json::to_string(&ErrorResponseBody::not_found(path, "category not found".to_string())).unwrap()
}