use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use crate::api::controller::connect::{get_connection, DbPool};
use crate::api::controller::helper::get_auth_user_from_request;
use crate::service::category_subcategories_service;
use crate::service::resource_mapper::category_resource_mapper;
use crate::api::error::ServiceError;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_subcategories);
    cfg.service(create_subcategory);
    cfg.service(delete_subcategory);
}

#[get("/categories/{id}/subcategories")]
async fn get_subcategories(path: web::Path<i64>, pool: web::Data<DbPool>, req: HttpRequest) -> Result<impl Responder, ServiceError> {
    let id = path.into_inner();
    let path = req.match_info().as_str();
    let auth_user = get_auth_user_from_request(&req)?;
    let mut connection = get_connection(pool, path)?;

    let result = category_subcategories_service::get_subcategories_for_category(&mut connection, &auth_user, id)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;

    let resources = category_resource_mapper::map_entities_to_category_resources(&mut connection, &auth_user, &result)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;
    Ok(HttpResponse::Ok().json(&resources))
}

#[post("/categories/{id}/subcategories/{childid}")]
async fn create_subcategory(path: web::Path<(i64, i64)>, pool: web::Data<DbPool>, req: HttpRequest) -> Result<impl Responder, ServiceError> {
    let (id, childid) = path.into_inner();
    let path = req.match_info().as_str();
    let auth_user = get_auth_user_from_request(&req)?;
    let mut connection = get_connection(pool, path)?;

    let result = category_subcategories_service::add_subcategory_to_category(&mut connection, &auth_user, id, childid)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;

    let resource = category_resource_mapper::map_entity_to_category_resource_hal(&mut connection, &auth_user, &result)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;
    Ok(HttpResponse::Created().json(&resource))
}

#[delete("/categories/{id}/subcategories/{childid}")]
async fn delete_subcategory(path: web::Path<(i64, i64)>, pool: web::Data<DbPool>, req: HttpRequest) -> Result<impl Responder, ServiceError> {
    let (id, childid) = path.into_inner();
    let path = req.match_info().as_str();
    let auth_user = get_auth_user_from_request(&req)?;
    let mut connection = get_connection(pool, path)?;

    category_subcategories_service::delete_subcategory_from_category(&mut connection, &auth_user, id, childid)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;
    Ok(HttpResponse::NoContent().finish())
}