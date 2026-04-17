use actix_web::{delete, get, options, post, put, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web::web::ServiceConfig;
use crate::api::dto::category_dto::{CreateCategoryDto, UpdateCategoryDto};
use crate::api::resource::category_resource::{CategoryResource, CategoryResourceHal};
use crate::dao::connect::connect;
use crate::security::auth_context_holder::AuthUser;
use crate::service::category_service;

// TODO: get the paths for responses out of the function definition
// TODO: validator doesn't seem to work correctly yet -> created empty name
// TODO: -> see how name==null works -> behaviour not explicit right now

// TODO: register all routes from controller here
pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(options_categories);
    cfg.service(get_categories);
    cfg.service(get_category);
    cfg.service(create_category);
    cfg.service(update_category);
    cfg.service(delete_category);
}

#[options("/categories")]
async fn options_categories() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("/categories")]
async fn get_categories(req: HttpRequest) -> impl Responder {
    let extensions = req.extensions();
    // TODO: remove unwrap
    let auth_user = extensions.get::<AuthUser>().unwrap();
    let mut connection = connect();

    let result = match category_service::get_categories(&mut connection, auth_user) {
        Ok(categories) => categories,
        Err(e) => return e.get_response("/categories".to_string())
    };

    let resources = result.iter().map(|r| CategoryResource::from_entity(&mut connection, r)).collect::<Vec<CategoryResource>>();
    HttpResponse::Ok().json(&resources)
}

#[get("/categories/{id}")]
async fn get_category(path: web::Path<i64>, req: HttpRequest) -> impl Responder {
    let id = path.into_inner();
    let extensions = req.extensions();
    // TODO: remove unwrap
    let auth_user = extensions.get::<AuthUser>().unwrap();
    let mut connection = connect();

    let result = match category_service::get_category_by_id(&mut connection, auth_user, id) {
        Ok(category) => category,
        Err(e) => return e.get_response(format!("/categories/{}", id))
    };

    let resource = CategoryResourceHal::from_entity(&mut connection, &result);
    HttpResponse::Ok().json(&resource)
}

#[post("/categories")]
async fn create_category(req: HttpRequest, new_category: web::Json<CreateCategoryDto>) -> impl Responder {
    let extensions = req.extensions();
    // TODO: remove unwrap
    let auth_user = extensions.get::<AuthUser>().unwrap();
    let mut connection = connect();

    let result = match category_service::create_category(&mut connection, auth_user, new_category.into_inner()) {
        Ok(category) => category,
        Err(e) => return e.get_response("/categories".to_string())
    };

    let resource = CategoryResourceHal::from_entity(&mut connection, &result);
    HttpResponse::Created().json(&resource)
}

#[put("/categories/{id}")]
async fn update_category(req: HttpRequest, path: web::Path<i64>, new_category: web::Json<UpdateCategoryDto>) -> impl Responder {
    let extensions = req.extensions();
    // TODO: remove unwrap
    let auth_user = extensions.get::<AuthUser>().unwrap();
    let id = path.into_inner();
    let new_category = new_category.into_inner();
    let mut connection = connect();

    let result = match category_service::update_category(&mut connection, auth_user, id, new_category) {
        Ok(category) => category,
        Err(e) => return e.get_response(format!("/categories/{}", id))
    };

    let resource = CategoryResourceHal::from_entity(&mut connection, &result);
    HttpResponse::Ok().json(&resource)
}

#[delete("/categories/{id}")]
async fn delete_category(req: HttpRequest, path: web::Path<i64>) -> impl Responder {
    let extensions = req.extensions();
    // TODO: remove unwrap
    let auth_user = extensions.get::<AuthUser>().unwrap();
    let id = path.into_inner();
    let mut connection = connect();

    match category_service::delete_category(&mut connection, auth_user, id) {
        Ok(_) => (),
        Err(e) => return e.get_response(format!("/categories/{}", id))
    };

    HttpResponse::NoContent().finish()
}