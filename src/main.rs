use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::error::{InternalError, JsonPayloadError};
use actix_web::http::Method;
use e_commerce_manual_translation::api::controller::{authentication_controller, category_controller, category_products_controller, category_subcategories_controller, product_controller};
use e_commerce_manual_translation::config::env_loader::{set_loader, LOADER};
use e_commerce_manual_translation::errors::error_enum::ErrorsEnum;
use e_commerce_manual_translation::security::auth_context_holder::AuthContextHolder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_loader()?;

    // TODO: add configurations
    HttpServer::new(|| {
        App::new()
            .app_data(
                web::JsonConfig::default().error_handler(|err, req|
                    handle_json_error(err, req).into()
                )
            )
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
    // FIXME: remove unwrap
    .bind(LOADER.get().unwrap().get_address())?
    .run()
    .await
}

fn handle_json_error(err: JsonPayloadError, req: &HttpRequest) -> InternalError<JsonPayloadError> {
    let msg = err.to_string();
    InternalError::from_response(err, ErrorsEnum::JsonParsingError(msg).get_response(req.path()))
}

fn default_handler(req: HttpRequest) -> impl Responder {
    if req.resource_map().has_resource(req.path()) {
        if req.method() == Method::OPTIONS {
            return HttpResponse::Ok().finish();
        }
        HttpResponse::MethodNotAllowed().finish()
    } else {
        HttpResponse::ImATeapot().finish()
    }
}