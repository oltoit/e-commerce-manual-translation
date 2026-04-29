use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use crate::api::controller::connect::{get_connection, DbPool};
use crate::api::dto::category_dto::{CreateCategoryDto, UpdateCategoryDto};
use crate::api::resource::category_resource::{CategoryResource, CategoryResourceHal};
use crate::service::category_service;
use crate::shared::auth::auth_user::AuthUser;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_categories);
    cfg.service(get_category);
    cfg.service(create_category);
    cfg.service(update_category);
    cfg.service(delete_category);
}

#[get("/categories")]
async fn get_categories(req: HttpRequest, pool: web::Data<DbPool>) -> impl Responder {
    let path = req.match_info().as_str();
    let auth_user = match AuthUser::get(&req) {
        Ok(user) => user,
        Err(e) => return e.get_response(path)
    };
    let mut connection = match get_connection(pool, path) {
        Ok(conn) => conn,
        Err(response) => return response
    };

    let result = match category_service::get_categories(&mut connection, &auth_user) {
        Ok(categories) => categories,
        Err(e) => return e.get_response(path)
    };

    let resources = match CategoryResource::map_from_entities(&mut connection, &auth_user, &result) {
        Ok(resources) => resources,
        Err(e) => return e.get_response(path)
    };
    HttpResponse::Ok().json(&resources)
}

#[get("/categories/{id}")]
async fn get_category(path: web::Path<i64>, pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
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

    let result = match category_service::get_category_by_id(&mut connection, &auth_user, id) {
        Ok(category) => category,
        Err(e) => return e.get_response(path)
    };

    let resource = match CategoryResourceHal::from_entity(&mut connection, &auth_user, &result) {
        Ok(resource) => resource,
        Err(e) => return e.get_response(path)
    };
    HttpResponse::Ok().json(&resource)
}

#[post("/categories")]
async fn create_category(req: HttpRequest, pool: web::Data<DbPool>, new_category: web::Json<CreateCategoryDto>) -> impl Responder {
    let path = req.match_info().as_str();
    let auth_user = match AuthUser::get(&req) {
        Ok(user) => user,
        Err(e) => return e.get_response(path)
    };
    let mut connection = match get_connection(pool, path) {
        Ok(conn) => conn,
        Err(response) => return response
    };

    let result = match category_service::create_category(&mut connection, &auth_user, new_category.into_inner()) {
        Ok(category) => category,
        Err(e) => return e.get_response(path)
    };

    let resource = match CategoryResourceHal::from_entity(&mut connection, &auth_user, &result) {
        Ok(resource) => resource,
        Err(e) => return e.get_response(path)
    };
    HttpResponse::Created().json(&resource)
}

#[put("/categories/{id}")]
async fn update_category(req: HttpRequest, path: web::Path<i64>, pool: web::Data<DbPool>, new_category: web::Json<UpdateCategoryDto>) -> impl Responder {
    let id = path.into_inner();
    let path = req.match_info().as_str();
    let auth_user = match AuthUser::get(&req) {
        Ok(user) => user,
        Err(e) => return e.get_response(path)
    };
    let new_category = new_category.into_inner();
    let mut connection = match get_connection(pool, path) {
        Ok(conn) => conn,
        Err(response) => return response
    };


    let result = match category_service::update_category(&mut connection, &auth_user, id, new_category) {
        Ok(category) => category,
        Err(e) => return e.get_response(path)
    };

    let resource = match CategoryResourceHal::from_entity(&mut connection, &auth_user, &result) {
        Ok(resource) => resource,
        Err(e) => return e.get_response(path)
    };
    HttpResponse::Ok().json(&resource)
}

#[delete("/categories/{id}")]
async fn delete_category(req: HttpRequest, path: web::Path<i64>, pool: web::Data<DbPool>) -> impl Responder {
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

    match category_service::delete_category(&mut connection, &auth_user, id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.get_response(path)
    }
}