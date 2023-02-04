use std::sync::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
mod book_controller;
use book_controller::{book, Books};

const HOST: &str = "localhost";
const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // create shared state for a book collection
    let books = web::Data::new(Mutex::new(Books::new()));

    // add sample book
    books.lock().unwrap().add(book::Book::new(
        "9781098122539".to_string(),
        "The Rust Programming Language".to_string(),
        2018,
        544,
    ));
    // one more
    books.lock().unwrap().add(book::Book::new(
        "67834187613".to_string(),
        "Some Other Book".to_string(),
        2019,
        544,
    ));

    #[derive(OpenApi)]
    //#[openapi(components(schemas(book::Book)), paths(book_controller::*))]
    #[openapi(
        info(
            description = "Api for a book colelction",
            version = "1.0.0",
            title = "Book API",
            license(name = "MIT", url = "https://opensource.org/licenses/MIT"),
            contact(
                name = "John Doe",
                email = "jondoe@jejeje.deeee",
                url = "https://www.jejeje.deeee"
            )
        ),
        paths(
            book_controller::get_books,
            book_controller::get_book_by_id,
            book_controller::remove_book,
            book_controller::add_book,
            book_controller::get_books_count
        ),
        components(schemas(book::Book))
    )]
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
    HttpResponse::Ok().body("Hello world!")
}
