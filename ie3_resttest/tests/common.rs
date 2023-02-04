use actix_web::web;
use std::sync::Mutex;

use ie3_resttest::{
    self,
    book_controller::{self, book, Books},
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    // create a shared data object
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

    // add data and routes to service
    cfg.app_data(books.clone());
    cfg.configure(book_controller::configure);
}
