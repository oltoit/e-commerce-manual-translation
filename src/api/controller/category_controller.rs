use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use crate::api::controller::connection_helper::{get_connection, DbPool};
use crate::api::controller::request_helper::get_auth_user_from_request;
use crate::api::dto::category_dto::{CreateCategoryDto, UpdateCategoryDto};
use crate::api::dto::validation_helper::validate_dto;
use crate::service::category_service;
use crate::service::resource_mapper::category_resource_mapper;
use crate::api::error::ServiceError;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_categories);
    cfg.service(get_category);
    cfg.service(create_category);
    cfg.service(update_category);
    cfg.service(delete_category);
}

#[get("/categories")]
async fn get_categories(req: HttpRequest, pool: web::Data<DbPool>) -> Result<impl Responder, ServiceError> {
    let path = req.match_info().as_str();
    let auth_user = get_auth_user_from_request(&req)?;
    let mut connection = get_connection(pool, path)?;

    let result = category_service::get_categories(&mut connection, &auth_user)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;

    let resources = category_resource_mapper::map_entities_to_category_resources(&mut connection, &auth_user, &result)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;
    Ok(HttpResponse::Ok().json(&resources))
}

#[get("/categories/{id}")]
async fn get_category(path: web::Path<i64>, pool: web::Data<DbPool>, req: HttpRequest) -> Result<impl Responder, ServiceError> {
    let id = path.into_inner();
    let path = req.match_info().as_str();
    let auth_user = get_auth_user_from_request(&req)?;
    let mut connection = get_connection(pool, path)?;

    let result = category_service::get_category_by_id(&mut connection, &auth_user, id)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;

    let resource = category_resource_mapper::map_entity_to_category_resource_hal(&mut connection, &auth_user, &result)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;
    Ok(HttpResponse::Ok().json(&resource))
}

#[post("/categories")]
async fn create_category(req: HttpRequest, pool: web::Data<DbPool>, new_category: web::Json<CreateCategoryDto>) -> Result<impl Responder, ServiceError> {
    let path = req.match_info().as_str();
    let new_category = new_category.into_inner();
    let new_category = validate_dto(&new_category, path)?.to_new_category();
    let auth_user = get_auth_user_from_request(&req)?;
    let mut connection = get_connection(pool, path)?;

    let result = category_service::create_category(&mut connection, &auth_user, new_category)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;

    let resource = category_resource_mapper::map_entity_to_category_resource_hal(&mut connection, &auth_user, &result)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;
    Ok(HttpResponse::Created().json(&resource))
}

#[put("/categories/{id}")]
async fn update_category(req: HttpRequest, path: web::Path<i64>, pool: web::Data<DbPool>, update_category: web::Json<UpdateCategoryDto>) -> Result<impl Responder, ServiceError> {
    let id = path.into_inner();
    let path = req.match_info().as_str();
    let update_category = update_category.into_inner();
    let update_category = validate_dto(&update_category, path)?.to_update_category();
    let auth_user = get_auth_user_from_request(&req)?;
    let mut connection = get_connection(pool, path)?;

    let result = category_service::update_category(&mut connection, &auth_user, id, update_category)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;

    let resource = category_resource_mapper::map_entity_to_category_resource_hal(&mut connection, &auth_user, &result)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;
    Ok(HttpResponse::Ok().json(&resource))
}

#[delete("/categories/{id}")]
async fn delete_category(req: HttpRequest, path: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder, ServiceError> {
    let id = path.into_inner();
    let path = req.match_info().as_str();
    let auth_user = get_auth_user_from_request(&req)?;
    let mut connection = get_connection(pool, path)?;

    match category_service::delete_category(&mut connection, &auth_user, id) {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => Err(ServiceError::new(path.to_string(), e))
    }
}