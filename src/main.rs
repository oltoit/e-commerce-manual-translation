use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web::error::{InternalError, JsonPayloadError};
use e_commerce_manual_translation::api::controller::{authentication_controller, category_controller};
use e_commerce_manual_translation::config::env_loader::{set_loader, LOADER};
use e_commerce_manual_translation::errors::error_enum::ErrorsEnum;
use e_commerce_manual_translation::security::auth_context_holder::AuthContextHolder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // initiate the EnvLoader with environment variables
    set_loader()?;

    // TODO: add configurations
    HttpServer::new(|| {
        App::new()
            .app_data(
                web::JsonConfig::default().error_handler(|err, req|
                    handle_json_error(err, req).into()
                )
            ).configure(authentication_controller::config)
            .service(
                web::scope("")
                    .wrap(AuthContextHolder)
                    .configure(category_controller::config)
            )
            .default_service(web::route().to(|| async {
                HttpResponse::ImATeapot().finish()
            }))
    })
        // FIXME: remove unwrap
        .bind(LOADER.get().unwrap().get_adress())?
        .run()
        .await
}

fn handle_json_error(err: JsonPayloadError, req: &HttpRequest) -> InternalError<JsonPayloadError> {
    let msg = err.to_string();
    InternalError::from_response(err, ErrorsEnum::JsonParsingError(msg).get_response(req.path()))
}