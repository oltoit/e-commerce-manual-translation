use crate::service::category_products_service;
use actix_web::{delete, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use serde_qs::actix::QsQuery;
use crate::api::controller::connect::connect;
use crate::api::controller::pagination::{get_optional_pagination, Pagination};
use crate::api::resource::product_resource::{ProductResource, ProductsResource};
use crate::security::auth_context_holder::AuthUser;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_products_for_category);
    cfg.service(add_product_to_category);
    cfg.service(remove_product_from_category);
}

#[get("/categories/{categoryid}/products")]
async fn get_products_for_category(pagination: Option<QsQuery<Pagination>>, path: web::Path<i64>, req: HttpRequest) -> impl Responder {
    let extensions = req.extensions();
    // TODO: remove unwrap
    let auth_user = extensions.get::<AuthUser>().unwrap();
    let category_id = path.into_inner();
    let pagination = get_optional_pagination(pagination);
    let mut connection = connect();

    let (result, total_elements) = match category_products_service::get_products_for_category(&mut connection, &auth_user, &pagination, category_id) {
        Ok(products) => products,
        Err(e) => return e.get_response(req.match_info().as_str())
    };

    let resources = match ProductsResource::new(&mut connection, auth_user, &result, &pagination, total_elements) {
        Ok(resources) => resources,
        Err(e) => return e.get_response(req.match_info().as_str())
    };
    HttpResponse::Ok().json(resources)
}

#[post("/categories/{categoryid}/products/{productid}")]
async fn add_product_to_category(path: web::Path<(i64, i64)>, req: HttpRequest) -> impl Responder {
    let extensions = req.extensions();
    // TODO: remove unwrap
    let auth_user = extensions.get::<AuthUser>().unwrap();
    let (category_id, product_id) = path.into_inner();
    let mut connection = connect();

    let result = match category_products_service::add_product_to_category(&mut connection, &auth_user, category_id, product_id) {
        Ok(product) => product,
        Err(e) => return e.get_response(req.match_info().as_str())
    };

    let resource = match ProductResource::from_product(&mut connection, auth_user, &result) {
        Ok(resource) => resource,
        Err(e) => return e.get_response(req.match_info().as_str())
    };
    HttpResponse::Created().json(resource)
}

#[delete("/categories/{categoryid}/products/{productid}")]
async fn remove_product_from_category(path: web::Path<(i64, i64)>, req: HttpRequest) -> impl Responder {
    let extensions = req.extensions();
    // TODO: remove unwrap
    let auth_user = extensions.get::<AuthUser>().unwrap();
    let (category_id, product_id) = path.into_inner();
    let mut connection = connect();

    match category_products_service::remove_product_from_category(&mut connection, &auth_user, category_id, product_id) {
        Ok(product) => product,
        Err(e) => return e.get_response(req.match_info().as_str())
    };
    HttpResponse::NoContent().finish()
}