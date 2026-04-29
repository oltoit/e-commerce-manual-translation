use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use crate::api::controller::connect::{get_connection, DbPool};
use crate::service::category_subcategories_service;
use crate::service::resource_mapper::category_resource_mapper;
use crate::shared::auth::auth_user::AuthUser;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_subcategories);
    cfg.service(create_subcategory);
    cfg.service(delete_subcategory);
}

#[get("/categories/{id}/subcategories")]
async fn get_subcategories(path: web::Path<i64>, pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let id = path.into_inner();
    let path = req.match_info().as_str();
    let auth_user = match AuthUser::get(&req) {
        Ok(user) => user,
        Err(e) => return e.get_response(path)
    };
    let mut connection = match get_connection(pool, path) {
        Ok(conn) => conn,
        Err(response) => return response
    };

    let result = match category_subcategories_service::get_subcategories_for_category(&mut connection, &auth_user, id) {
        Ok(categories) => categories,
        Err(e) => return e.get_response(path)
    };

    let resources = match category_resource_mapper::map_entities_to_category_resources(&mut connection, &auth_user, &result) {
        Ok(resources) => resources,
        Err(e) => return e.get_response(path)
    };
    HttpResponse::Ok().json(&resources)
}

#[post("/categories/{id}/subcategories/{childid}")]
async fn create_subcategory(path: web::Path<(i64, i64)>, pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let (id, childid) = path.into_inner();
    let path = req.match_info().as_str();
    let auth_user = match AuthUser::get(&req) {
        Ok(user) => user,
        Err(e) => return e.get_response(path)
    };
    let mut connection = match get_connection(pool, path) {
        Ok(conn) => conn,
        Err(response) => return response
    };

    let result = match category_subcategories_service::add_subcategory_to_category(&mut connection, &auth_user, id, childid) {
        Ok(category) => category,
        Err(e) => return e.get_response(path)
    };

    let resource = match category_resource_mapper::map_entity_to_category_resource_hal(&mut connection, &auth_user, &result) {
        Ok(resource) => resource,
        Err(e) => return e.get_response(path)
    };
    HttpResponse::Created().json(&resource)
}

#[delete("/categories/{id}/subcategories/{childid}")]
async fn delete_subcategory(path: web::Path<(i64, i64)>, pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let (id, childid) = path.into_inner();
    let path = req.match_info().as_str();
    let auth_user = match AuthUser::get(&req) {
        Ok(user) => user,
        Err(e) => return e.get_response(path)
    };
    let mut connection = match get_connection(pool, path) {
        Ok(conn) => conn,
        Err(response) => return response
    };

    match category_subcategories_service::delete_subcategory_from_category(&mut connection, &auth_user, id, childid) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.get_response(path)
    }
}