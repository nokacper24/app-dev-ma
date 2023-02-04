use actix_web::{delete, get, post, web, HttpResponse, Responder};
use log::{info, log};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::format,
    sync::{Mutex, RwLock},
};
use utoipa::{openapi::info, ToSchema};

use self::book::Book;
pub mod book;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Books {
    books: RwLock<HashMap<String, Book>>,
}
impl Books {
    pub fn new() -> Self {
        Books {
            books: RwLock::new(HashMap::new()),
        }
    }

    pub fn add(&mut self, book: Book) {
        let mut map = self.books.write().unwrap();
        map.insert(book.id().clone(), book);
    }

    fn get(&self, id: String) -> Option<Book> {
        let map = self.books.read().unwrap();
        map.get(&id).cloned()
    }

    fn remove(&mut self, id: &str) -> Option<Book> {
        let mut map = self.books.write().unwrap();
        map.remove(id)
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_books_count); // has to be above get_book_by_id, otherwise it will look for book with id "count"
                                  // feels like bad practice.  Maybe there's a better way to do this? Use query strings for search?
    cfg.service(get_books);
    cfg.service(add_book);
    cfg.service(get_book_by_id);
    cfg.service(remove_book);
}

#[utoipa::path(
    responses(
    (status = 200, description = "List all books", body = Vec<Book>)
)
)]
#[get("/books")]
async fn get_books(books: web::Data<Mutex<Books>>) -> impl Responder {
    let books = books.lock().unwrap();
    let json_books: Vec<Book> = books.books.read().unwrap().values().cloned().collect();
    HttpResponse::Ok().json(json_books)
}

#[utoipa::path(
    responses(
    (status = 200, description = "Returns a book by id", body = Book),
    (status = 404, description = "Book not found"),
),
    params(
        ("id", description = "The id of the book"),
    ),
)]
#[get("/books/{id}")]
async fn get_book_by_id(books: web::Data<Mutex<Books>>, id: web::Path<String>) -> impl Responder {
    match books.lock() {
        Ok(guard) => match guard.get(id.to_string()) {
            Some(book) => HttpResponse::Ok().json(book),
            None => HttpResponse::NotFound().body("Book not found"),
        },
        Err(poisoned) => HttpResponse::InternalServerError()
            .body(format!("Error accessing books data: {}", poisoned)),
    }
}

#[utoipa::path(
    responses(
    (status = 201, description = "Add a book", body = Book),
),
    request_body = Book,
)]
#[post("/books")]
async fn add_book(books: web::Data<Mutex<Books>>, book: web::Json<Book>) -> impl Responder {
    let book = book.into_inner();
    let mut guard = match books.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error accessing books data: {}", poisoned))
        }
    };
    guard.add(book.clone());
    HttpResponse::Created().json(book)
}

#[utoipa::path(
    responses(
    (status = 200, description = "Removes a book", body = String),
    (status = 404, description = "Book not found"),
),
    params(
        ("id", description = "The id of the book to remove",)
    )
)]
#[delete("/books/{id}")]
async fn remove_book(books: web::Data<Mutex<Books>>, id: web::Path<String>) -> impl Responder {
    let mut books = match books.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            return HttpResponse::InternalServerError()
                .body(format!("Error accessing books data: {}", poisoned))
        }
    };
    match books.remove(&id) {
        Some(book) => HttpResponse::Ok().body(format!("Removed book {}", book.title())),
        None => HttpResponse::NotFound().body("Book not found"),
    }
}

#[utoipa::path(
    responses(
    (status = 200, description = "Returns the number of books", body = String),
),
)]
#[get("/books/count")]
async fn get_books_count(books: web::Data<Mutex<Books>>) -> impl Responder {
    let books = match books.lock() {
        Ok(guard) => guard,
        Err(poisoned) => {
            info!("Error accessing books data: {}", poisoned);
            return HttpResponse::InternalServerError()
                .body(format!("Error accessing books data: {}", poisoned));
        }
    };
    let count = match books.books.read() {
        Ok(guard) => guard.len(),
        Err(poisoned) => {
            info!("Error accessing books data: {}", poisoned);
            return HttpResponse::InternalServerError()
                .body(format!("Error accessing books data: {}", poisoned));
        }
    };
    HttpResponse::Ok().body(format!("{}", count))
}
