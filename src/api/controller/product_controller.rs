use crate::service::product_service;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use crate::api::controller::pagination::{get_optional_pagination, Pagination};
use crate::api::dto::product_dto::{CreateProductDto, UpdateProductDto};
use serde_qs::actix::QsQuery;
use validator::Validate;
use crate::api::controller::connect::{get_connection, DbPool};
use crate::service::resource_mapper::product_resource_mapper;
use crate::shared::auth::auth_user::AuthUser;
use crate::shared::errors::error_enum::ErrorsEnum;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_products);
    cfg.service(get_product);
    cfg.service(create_product);
    cfg.service(update_product);
    cfg.service(delete_product);
}

#[get("/products")]
async fn get_products(pagination: Option<QsQuery<Pagination>>, pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let path = req.match_info().as_str();
    let auth_user = match AuthUser::get(&req) {
        Ok(user) => user,
        Err(e) => return e.get_response(path)
    };
    let pagination = get_optional_pagination(pagination);
    let mut connection = match get_connection(pool, path) {
        Ok(conn) => conn,
        Err(response) => return response
    };

    let (result, total_elements) = match product_service::get_products_with_users(&mut connection, &auth_user, &pagination) {
        Ok(result) => result,
        Err(e) => return e.get_response(path),
    };

    let resource = match product_resource_mapper::map_entity_to_products_resource(&mut connection, &auth_user, &result, &pagination, &req, total_elements) {
        Ok(resource) => resource,
        Err(e) => return e.get_response(path),
    };
    HttpResponse::Ok().json(resource)
}

#[get("/products/{id}")]
async fn get_product(path: web::Path<i64>, pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
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

    let result = match product_service::get_product_with_user_by_id(&mut connection, &auth_user, id) {
        Ok(product) => product,
        Err(e) => return e.get_response(path)
    };

    let resource = match product_resource_mapper::map_entity_to_product_resource(&mut connection, &auth_user, &result) {
        Ok(resource) => resource,
        Err(e) => return e.get_response(path)
    };
    HttpResponse::Ok().json(&resource)
}

#[post("/products")]
async fn create_product(req: HttpRequest, pool: web::Data<DbPool>, create_product: web::Json<CreateProductDto>) -> impl Responder {
    let path = req.match_info().as_str();
    let auth_user = match AuthUser::get(&req) {
        Ok(user) => user,
        Err(e) => return e.get_response(path)
    };
    let create_product = create_product.into_inner();
    match create_product.validate() {
        Ok(_) => (),
        Err(e) => return ErrorsEnum::DTONotValid(e.to_string()).get_response(path)
    };
    let new_product = create_product.to_new_product(auth_user.id);

    let mut connection = match get_connection(pool, path) {
        Ok(conn) => conn,
        Err(response) => return response
    };

    let result = match product_service::create_product(
        &mut connection,
        &auth_user,
        new_product,
        create_product.currency.as_str()
    ).await {
        Ok(product) => product,
        Err(e) => return e.get_response(path)
    };

    let resource = match product_resource_mapper::map_entity_to_product_resource(&mut connection, &auth_user, &result) {
        Ok(resource) => resource,
        Err(e) => return e.get_response(path)
    };
    HttpResponse::Created().json(&resource)
}

#[put("/products/{id}")]
async fn update_product(req: HttpRequest, path: web::Path<i64>, pool: web::Data<DbPool>, update_product: web::Json<UpdateProductDto>) -> impl Responder {
    let id = path.into_inner();
    let path = req.match_info().as_str();
    let auth_user = match AuthUser::get(&req) {
        Ok(user) => user,
        Err(e) => return e.get_response(path)
    };
    let update_product = update_product.into_inner();
    match update_product.validate() {
        Ok(_) => (),
        Err(e) => return ErrorsEnum::DTONotValid(e.to_string()).get_response(path)
    }
    let update_product_entity = update_product.to_update_product(auth_user.id);
    let mut connection = match get_connection(pool, path) {
        Ok(conn) => conn,
        Err(response) => return response
    };

    let result = match product_service::update_product(
        &mut connection,
        &auth_user,
        update_product_entity,
        update_product.currency.as_str(),
        id
    ).await {
        Ok(product) => product,
        Err(e) => return e.get_response(path)
    };

    let resource = match product_resource_mapper::map_entity_to_product_resource(&mut connection, &auth_user, &result) {
        Ok(resource) => resource,
        Err(e) => return e.get_response(path)
    };
    HttpResponse::Ok().json(&resource)
}

#[delete("/products/{id}")]
async fn delete_product(req: HttpRequest, path: web::Path<i64>, pool: web::Data<DbPool>) -> impl Responder {
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

    match product_service::delete_product(&mut connection, &auth_user, id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.get_response(path)
    }
}
