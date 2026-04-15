use actix_web::{App, HttpServer};
use e_commerce_manual_translation::api::controller::category_controller;
use e_commerce_manual_translation::config::env_loader::{set_loader, LOADER};

// TODO: async routing
// TODO: authentication and authorization
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match set_loader() {
        Ok(_) => (),
        Err(e) => return Err(e)
    }

    // TODO: add configurations
    HttpServer::new(|| {
        App::new()
            .configure(category_controller::config)
    })
        // FIXME: remove unwrap
        .bind(LOADER.get().unwrap().get_adress())?
        .run()
        .await
}