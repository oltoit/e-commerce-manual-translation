use actix_web::{web, App, HttpResponse, HttpServer};
use e_commerce_manual_translation::api::controller::{authentication_controller, category_controller};
use e_commerce_manual_translation::config::env_loader::{set_loader, LOADER};
use e_commerce_manual_translation::security::auth_context_holder::AuthContextHolder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // initiate the EnvLoader with environment variables
    set_loader()?;

    // TODO: add configurations
    HttpServer::new(|| {
        App::new()
            .configure(authentication_controller::config)
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