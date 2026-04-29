use crate::service::category_products_service;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use serde_qs::actix::QsQuery;
use crate::api::controller::connect::{get_connection, DbPool};
use crate::api::controller::pagination::{get_optional_pagination, Pagination};
use crate::service::resource_mapper::product_resource_mapper;
use crate::shared::auth::auth_user::AuthUser;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_products_for_category);
    cfg.service(add_product_to_category);
    cfg.service(remove_product_from_category);
}

#[get("/categories/{categoryid}/products")]
async fn get_products_for_category(pagination: Option<QsQuery<Pagination>>, path: web::Path<i64>, pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let category_id = path.into_inner();
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

    let (result, total_elements) = match category_products_service::get_products_for_category(&mut connection, &auth_user, &pagination, category_id) {
        Ok(products) => products,
        Err(e) => return e.get_response(path)
    };

    let resources = match product_resource_mapper::map_entity_to_products_resource(&mut connection, &auth_user, &result, &pagination,  &req, total_elements) {
        Ok(resources) => resources,
        Err(e) => return e.get_response(path)
    };
    HttpResponse::Ok().json(resources)
}

#[post("/categories/{categoryid}/products/{productid}")]
async fn add_product_to_category(path: web::Path<(i64, i64)>, pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let (category_id, product_id) = path.into_inner();
    let path = req.match_info().as_str();
    let auth_user = match AuthUser::get(&req) {
        Ok(user) => user,
        Err(e) => return e.get_response(path)
    };
    let mut connection = match get_connection(pool, path) {
        Ok(conn) => conn,
        Err(response) => return response
    };

    let result = match category_products_service::add_product_to_category(&mut connection, &auth_user, category_id, product_id) {
        Ok(product) => product,
        Err(e) => return e.get_response(path)
    };

    let resource = match product_resource_mapper::map_entity_to_product_resource(&mut connection, &auth_user, &result) {
        Ok(resource) => resource,
        Err(e) => return e.get_response(path)
    };
    HttpResponse::Created().json(resource)
}

#[delete("/categories/{categoryid}/products/{productid}")]
async fn remove_product_from_category(path: web::Path<(i64, i64)>, pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
    let (category_id, product_id) = path.into_inner();
    let path = req.match_info().as_str();
    let auth_user = match AuthUser::get(&req) {
        Ok(user) => user,
        Err(e) => return e.get_response(path)
    };
    let mut connection = match get_connection(pool, path) {
        Ok(conn) => conn,
        Err(response) => return response
    };

    match category_products_service::remove_product_from_category(&mut connection, &auth_user, category_id, product_id) {
        Ok(product) => product,
        Err(e) => return e.get_response(path)
    };
    HttpResponse::NoContent().finish()
}