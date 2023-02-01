use std::sync::Mutex;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
mod book_controller;
use book_controller::{book, Books};

const HOST: &str = "localhost";
const PORT: u16 = 8080;

/// Starting point for the application.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let books = web::Data::new(Mutex::new(Books::new()));

    // add sample book
    books.lock().unwrap().add(book::Book::new(
        "9781098122539".to_string(),
        "The Rust Programming Language".to_string(),
        2018,
        "544".to_string(),
    ));



    #[derive(OpenApi)]
    //#[openapi(components(schemas(book::Book)), paths(book_controller::*))]
    #[openapi(paths(book_controller::get_books, book_controller::get_book_by_id), components(schemas(book::Book)))]
    struct ApiDoc;

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(books.clone())
            .wrap(logger)
            .configure(book_controller::configure)
            .service(index)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind((HOST, PORT))?
    .run()
    .await
}

#[get("/")]
async fn index() -> impl Responder {
    println!("index");
    HttpResponse::BadRequest().body("Bad Request")
}
