use actix_web::{web, App, HttpServer};
mod hello_controller;
mod not_found;

const HOST_NAME: &str = "localhost";
const PORT: u16 = 8080;

/// Starting point for the application.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(hello_controller::configure)
            .default_service(web::route().to(not_found::not_found))
    })
    .bind((HOST_NAME, PORT))?
    .run()
    .await
}