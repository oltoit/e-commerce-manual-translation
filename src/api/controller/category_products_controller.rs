use crate::service::category_products_service;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use serde_qs::actix::QsQuery;
use crate::api::controller::connection_helper::{get_connection, DbPool};
use crate::api::controller::request_helper::get_auth_user_from_request;
use crate::api::controller::pagination_helper::{get_optional_pagination, Pagination};
use crate::service::resource_mapper::product_resource_mapper;
use crate::api::error::ServiceError;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_products_for_category);
    cfg.service(add_product_to_category);
    cfg.service(remove_product_from_category);
}

#[get("/categories/{categoryid}/products")]
async fn get_products_for_category(pagination: Option<QsQuery<Pagination>>, path: web::Path<i64>, pool: web::Data<DbPool>, req: HttpRequest) -> Result<impl Responder, ServiceError> {
    let category_id = path.into_inner();
    let path = req.match_info().as_str();
    let auth_user = get_auth_user_from_request(&req)?;
    let pagination = get_optional_pagination(pagination);
    let mut connection = get_connection(pool, path)?;

    let (result, total_elements) = category_products_service::get_products_for_category(&mut connection, &auth_user, &pagination, category_id)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;

    let resources = product_resource_mapper::map_entity_to_products_resource(&mut connection, &auth_user, &result, &pagination,  &req, total_elements)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;
    Ok(HttpResponse::Ok().json(resources))
}

#[post("/categories/{categoryid}/products/{productid}")]
async fn add_product_to_category(path: web::Path<(i64, i64)>, pool: web::Data<DbPool>, req: HttpRequest) -> Result<impl Responder, ServiceError> {
    let (category_id, product_id) = path.into_inner();
    let path = req.match_info().as_str();
    let auth_user = get_auth_user_from_request(&req)?;
    let mut connection = get_connection(pool, path)?;

    let result = category_products_service::add_product_to_category(&mut connection, &auth_user, category_id, product_id)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;

    let resource = product_resource_mapper::map_entity_to_product_resource(&mut connection, &auth_user, &result)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;
    Ok(HttpResponse::Created().json(resource))
}

#[delete("/categories/{categoryid}/products/{productid}")]
async fn remove_product_from_category(path: web::Path<(i64, i64)>, pool: web::Data<DbPool>, req: HttpRequest) -> Result<impl Responder, ServiceError> {
    let (category_id, product_id) = path.into_inner();
    let path = req.match_info().as_str();
    let auth_user = get_auth_user_from_request(&req)?;
    let mut connection = get_connection(pool, path)?;

    category_products_service::remove_product_from_category(&mut connection, &auth_user, category_id, product_id)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;
    Ok(HttpResponse::NoContent().finish())
}