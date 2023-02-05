use actix_web::{http::StatusCode, test, App};
use ie3_resttest::book_controller::book_collection::book::Book;

mod common;

#[actix_web::test]
async fn test_get_book_count() {
    let app = test::init_service(App::new().configure(common::configure)).await;

    let req = test::TestRequest::get().uri("/books/count").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);
    let body = test::read_body(resp).await;
    assert_eq!(body, "2");
}

#[actix_web::test]
async fn test_get_invalid_book() {
    let app = test::init_service(App::new().configure(common::configure)).await;
    let req = test::TestRequest::get().uri("/books/883").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_get_bok() {
    let app = test::init_service(App::new().configure(common::configure)).await;
    let req = test::TestRequest::get()
        .uri("/books/9781098122539")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let body = test::read_body(resp).await;
    let book = serde_json::from_slice::<Book>(&body).unwrap();

    assert_eq!(book.id(), "9781098122539");
    assert_eq!(book.title(), "The Rust Programming Language");
    assert_eq!(*book.year(), 2018);
    assert_eq!(*book.number_pages(), 544);
}

#[actix_web::test]
async fn test_delete_invalid() {
    let app = test::init_service(App::new().configure(common::configure)).await;
    let req = test::TestRequest::delete().uri("/books/883").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_elaborate_test() {
    let app = test::init_service(App::new().configure(common::configure)).await;

    // Get the book count
    let req = test::TestRequest::get().uri("/books/count").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body = test::read_body(resp).await;
    assert_eq!(body, "2");

    // Get a book
    let req = test::TestRequest::get()
        .uri("/books/9781098122539")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body = test::read_body(resp).await;
    let book = serde_json::from_slice::<Book>(&body).unwrap();
    assert_eq!(book.title(), "The Rust Programming Language");
    assert_eq!(*book.year(), 2018);
    assert_eq!(*book.number_pages(), 544);

    // Delete a book
    let req = test::TestRequest::delete()
        .uri("/books/9781098122539")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    // Try to get the deleted book, should fail
    let req = test::TestRequest::get()
        .uri("/books/9781098122539")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);

    // Get the book count
    let req = test::TestRequest::get().uri("/books/count").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body = test::read_body(resp).await;
    assert_eq!(body, "1");

    // Create a new book to add
    let new_book = Book::new(
        "9781098122539".to_string(),
        "The Rust Programming Language".to_string(),
        2018,
        544,
    );
    // Add the new book
    let req = test::TestRequest::post()
        .uri("/books")
        .set_json(&new_book)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Check if added book is equal to the new book
    let req = test::TestRequest::get()
        .uri("/books/9781098122539")
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body = test::read_body(resp).await;
    let book = serde_json::from_slice::<Book>(&body).unwrap();
    assert_eq!(new_book, book);
}
