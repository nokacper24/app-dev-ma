use actix_web::{test, App};

mod common;

#[actix_web::test]
async fn test_get_book_count() {
    let app = test::init_service(App::new().configure(common::configure)).await;

    let req = test::TestRequest::get().uri("/books/count").to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert_eq!(body, "2");
}
