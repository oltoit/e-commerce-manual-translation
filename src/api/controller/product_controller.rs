use crate::service::product_service;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use crate::api::controller::pagination::{get_optional_pagination, Pagination};
use crate::api::dto::product_dto::{CreateProductDto, UpdateProductDto};
use serde_qs::actix::QsQuery;
use crate::api::controller::connect::{get_connection, DbPool};
use crate::api::controller::helper::get_auth_user_from_request;
use crate::api::dto::validation_helper::validate_dto;
use crate::service::resource_mapper::product_resource_mapper;
use crate::api::error::ServiceError;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_products);
    cfg.service(get_product);
    cfg.service(create_product);
    cfg.service(update_product);
    cfg.service(delete_product);
}

#[get("/products")]
async fn get_products(pagination: Option<QsQuery<Pagination>>, pool: web::Data<DbPool>, req: HttpRequest) -> Result<impl Responder, ServiceError> {
    let path = req.match_info().as_str();
    let auth_user = get_auth_user_from_request(&req)?;
    let pagination = get_optional_pagination(pagination);
    let mut connection = get_connection(pool, path)?;

    let (result, total_elements) = product_service::get_products_with_users(&mut connection, &auth_user, &pagination)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;

    let resource = product_resource_mapper::map_entity_to_products_resource(&mut connection, &auth_user, &result, &pagination, &req, total_elements)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;
    Ok(HttpResponse::Ok().json(resource))
}

#[get("/products/{id}")]
async fn get_product(path: web::Path<i64>, pool: web::Data<DbPool>, req: HttpRequest) -> Result<impl Responder, ServiceError> {
    let id = path.into_inner();
    let path = req.match_info().as_str();
    let auth_user = get_auth_user_from_request(&req)?;
    let mut connection = get_connection(pool, path)?;

    let result = product_service::get_product_with_user_by_id(&mut connection, &auth_user, id)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;

    let resource = product_resource_mapper::map_entity_to_product_resource(&mut connection, &auth_user, &result)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;
    Ok(HttpResponse::Ok().json(&resource))
}

#[post("/products")]
async fn create_product(req: HttpRequest, pool: web::Data<DbPool>, create_product: web::Json<CreateProductDto>) -> Result<impl Responder, ServiceError> {
    let path = req.match_info().as_str();
    let auth_user = get_auth_user_from_request(&req)?;
    let create_product = create_product.into_inner();
    let new_product = validate_dto(&create_product, path)?.to_new_product(auth_user.id);
    let mut connection = get_connection(pool, path)?;

    let result = product_service::create_product(
        &mut connection,
        &auth_user,
        new_product,
        create_product.currency.as_str()
    )
        .await
        .map_err(|e| ServiceError::new(path.to_string(), e))?;

    let resource = product_resource_mapper::map_entity_to_product_resource(&mut connection, &auth_user, &result)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;
    Ok(HttpResponse::Created().json(&resource))
}

#[put("/products/{id}")]
async fn update_product(req: HttpRequest, path: web::Path<i64>, pool: web::Data<DbPool>, update_product: web::Json<UpdateProductDto>) -> Result<impl Responder, ServiceError> {
    let id = path.into_inner();
    let path = req.match_info().as_str();
    let auth_user = get_auth_user_from_request(&req)?;
    let update_product = update_product.into_inner();
    let update_product_entity = validate_dto(&update_product, path)?.to_update_product(auth_user.id);
    let mut connection = get_connection(pool, path)?;

    let result = product_service::update_product(
        &mut connection,
        &auth_user,
        update_product_entity,
        update_product.currency.as_str(),
        id
    )
        .await
        .map_err(|e| ServiceError::new(path.to_string(), e))?;

    let resource = product_resource_mapper::map_entity_to_product_resource(&mut connection, &auth_user, &result)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;
    Ok(HttpResponse::Ok().json(&resource))
}

#[delete("/products/{id}")]
async fn delete_product(req: HttpRequest, path: web::Path<i64>, pool: web::Data<DbPool>) -> Result<impl Responder, ServiceError> {
    let id = path.into_inner();
    let path = req.match_info().as_str();
    let auth_user = get_auth_user_from_request(&req)?;
    let mut connection = get_connection(pool, path)?;

    product_service::delete_product(&mut connection, &auth_user, id)
        .map_err(|e| ServiceError::new(path.to_string(), e))?;
    Ok(HttpResponse::NoContent().finish())
}
