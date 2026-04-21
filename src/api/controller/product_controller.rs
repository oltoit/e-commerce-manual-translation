use crate::service::product_service;
use crate::security::auth_context_holder::AuthUser;
use actix_web::{delete, get, post, put, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use crate::api::controller::connect::connect;
use crate::api::controller::pagination::{get_optional_pagination, Pagination};
use crate::api::dto::product_dto::{CreateProductDto, UpdateProductDto};
use crate::api::resource::product_resource::{ProductResource, ProductsResource};
use serde_qs::actix::QsQuery;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_products);
    cfg.service(get_product);
    cfg.service(create_product);
    cfg.service(update_product);
    cfg.service(delete_product);
}

// TODO: check if controller methods need to start their own transactions

#[get("/products")]
async fn get_products(pagination: Option<QsQuery<Pagination>>, req: HttpRequest) -> impl Responder {
    let extensions = req.extensions();
    // TODO: remove unwrap
    let auth_user = extensions.get::<AuthUser>().unwrap();
    let pagination = get_optional_pagination(pagination);
    let mut connection = connect();

    let (result, total_elements) = match product_service::get_products_with_users(&mut connection, &auth_user, &pagination) {
        Ok(result) => result,
        Err(e) => return e.get_response(req.match_info().as_str()),
    };

    let resource = match ProductsResource::new(&mut connection, auth_user, &result, &pagination, total_elements) {
        Ok(resource) => resource,
        Err(e) => return e.get_response(req.match_info().as_str()),
    };
    HttpResponse::Ok().json(resource)
}

#[get("/products/{id}")]
async fn get_product(path: web::Path<i64>, req: HttpRequest) -> impl Responder {
    let id = path.into_inner();
    let path = req.match_info().as_str();
    let extensions = req.extensions();
    // TODO: remove unwrap
    let auth_user = extensions.get::<AuthUser>().unwrap();
    let mut connection = connect();

    let result = match product_service::get_product_with_user_by_id(&mut connection, &auth_user, id) {
        Ok(product) => product,
        Err(e) => return e.get_response(path)
    };

    let resource = match ProductResource::from_product(&mut connection, &auth_user, &result) {
        Ok(resource) => resource,
        Err(e) => return e.get_response(path)
    };
    HttpResponse::Ok().json(&resource)
}

#[post("/products")]
async fn create_product(req: HttpRequest, new_category: web::Json<CreateProductDto>) -> impl Responder {
    let extensions = req.extensions();
    // TODO: remove unwrap
    let auth_user = extensions.get::<AuthUser>().unwrap();
    let mut connection = connect();

    let result = match product_service::create_product(&mut connection, &auth_user, new_category.into_inner()).await {
        Ok(product) => product,
        Err(e) => return e.get_response(req.match_info().as_str())
    };

    let resource = match ProductResource::from_product(&mut connection, &auth_user, &result) {
        Ok(resource) => resource,
        Err(e) => return e.get_response(req.match_info().as_str())
    };
    HttpResponse::Created().json(&resource)
}

#[put("/products/{id}")]
async fn update_product(req: HttpRequest, path: web::Path<i64>, new_product: web::Json<UpdateProductDto>) -> impl Responder {
    let id = path.into_inner();
    let path = req.match_info().as_str();
    let extensions = req.extensions();
    // TODO: remove unwrap
    let auth_user = extensions.get::<AuthUser>().unwrap();
    let mut connection = connect();

    let result = match product_service::update_product(&mut connection, &auth_user, new_product.into_inner(), id).await {
        Ok(product) => product,
        Err(e) => return e.get_response(path)
    };

    let resource = match ProductResource::from_product(&mut connection, &auth_user, &result) {
        Ok(resource) => resource,
        Err(e) => return e.get_response(path)
    };
    HttpResponse::Ok().json(&resource)
}

#[delete("/products/{id}")]
async fn delete_product(req: HttpRequest, path: web::Path<i64>) -> impl Responder {
    let id = path.into_inner();
    let extensions = req.extensions();
    // TODO: remove unwrap
    let auth_user = extensions.get::<AuthUser>().unwrap();
    let mut connection = connect();

    match product_service::delete_product(&mut connection, &auth_user, id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => e.get_response(req.match_info().as_str())
    }
}
