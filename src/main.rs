use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder, middleware::Logger};
use actix_web::error::{InternalError, JsonPayloadError};
use actix_web::http::Method;
use env_logger::Env;
use log::warn;
use e_commerce_manual_translation::api::controller::{authentication_controller, category_controller, category_products_controller, category_subcategories_controller, product_controller};
use e_commerce_manual_translation::api::controller::connect::create_pool;
use e_commerce_manual_translation::config::env_loader::{get_loader, set_loader};
use e_commerce_manual_translation::errors::error_enum::ErrorsEnum;
use e_commerce_manual_translation::security::auth_context_holder::AuthContextHolder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_loader()?;

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let connection_pool = create_pool().map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "error creating connection pool"))?;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(
                web::JsonConfig::default().error_handler(|err, req|
                    handle_json_error(err, req).into()
                )
            )
            .app_data(web::Data::new(connection_pool.clone()))
            .configure(authentication_controller::config)
            .service(
                web::scope("")
                    .wrap(AuthContextHolder)
                    .configure(category_controller::config)
                    .configure(category_subcategories_controller::config)
                    .configure(product_controller::config)
                    .configure(category_products_controller::config)
            ).default_service(web::to(|req: HttpRequest| async move {
                default_handler(req)
            }))
    })
    .bind(get_adress()?)?
    .run()
    .await
}

fn handle_json_error(err: JsonPayloadError, req: &HttpRequest) -> InternalError<JsonPayloadError> {
    let msg = err.to_string();
    InternalError::from_response(err, ErrorsEnum::JsonParsingError(msg).get_response(req.path()))
}

fn get_adress() -> Result<String, std::io::Error> {
    match get_loader() {
        Ok(loader) => Ok(loader.get_address()),
        Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "error loading env variables")),
    }
}

fn default_handler(req: HttpRequest) -> impl Responder {
    if req.resource_map().has_resource(req.path()) {
        if req.method() == Method::OPTIONS {
            return HttpResponse::Ok().finish();
        }
        warn!("Method {:?} not allowed on endpoint {}", req.method(), req.path());
        HttpResponse::MethodNotAllowed().finish()
    } else {
        warn!("Not Handler found for endpoint {}", req.path());
        HttpResponse::ImATeapot().finish()
    }
}