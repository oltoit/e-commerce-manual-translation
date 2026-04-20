use actix_web::{delete, get, options, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use crate::api::controller::connect::connect;
use crate::api::resource::category_resource::{CategoryResource, CategoryResourceHal};
use crate::security::auth_context_holder::AuthUser;
use crate::service::category_subcategories_service;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(options_subcategories);
    cfg.service(options_subcategory);
    cfg.service(get_subcategories);
    cfg.service(create_subcategory);
    cfg.service(delete_subcategory);
}

#[options("/categories/{id}/subcategories")]
async fn options_subcategories() -> impl Responder { HttpResponse::Ok().finish() }

#[options("/categories/{id}/subcategories/{childid}")]
async fn options_subcategory() -> impl Responder { HttpResponse::Ok().finish() }

#[get("/categories/{id}/subcategories")]
async fn get_subcategories(path: web::Path<i64>, req: HttpRequest) -> impl Responder {
    let id = path.into_inner();
    let extensions = req.extensions();
    let auth_user = extensions.get::<AuthUser>().unwrap();
    let mut connection = connect();

    let result = match category_subcategories_service::get_subcategories_for_category(&mut connection, &auth_user, id) {
        Ok(categories) => categories,
        Err(e) => return e.get_response(req.match_info().as_str())
    };

    let resources = result.iter().map(|r| CategoryResource::from_entity(&mut connection, r)).collect::<Vec<CategoryResource>>();
    HttpResponse::Ok().json(&resources)
}

#[post("/categories/{id}/subcategories/{childid}")]
async fn create_subcategory(path: web::Path<(i64, i64)>, req: HttpRequest) -> impl Responder {
    let (id, childid) = path.into_inner();
    let extensions = req.extensions();
    let auth_user = extensions.get::<AuthUser>().unwrap();
    let mut connection = connect();

    let result = match category_subcategories_service::add_subcategory_to_category(&mut connection, &auth_user, id, childid) {
        Ok(category) => category,
        Err(e) => return e.get_response(req.match_info().as_str())
    };

    let resource = CategoryResourceHal::from_entity(&mut connection, &result);
    HttpResponse::Created().json(&resource)
}

#[delete("/categories/{id}/subcategories/{childid}")]
async fn delete_subcategory(path: web::Path<(i64, i64)>, req: HttpRequest) -> impl Responder {
    let (id, childid) = path.into_inner();
    let extensions = req.extensions();
    let auth_user = extensions.get::<AuthUser>().unwrap();
    let mut connection = connect();

    match category_subcategories_service::delete_subcategory_from_category(&mut connection, &auth_user, id, childid) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.get_response(req.match_info().as_str())
    }
}