use actix_web::{get, HttpResponse, Responder, delete, web};

/// This function is called when the route /hello is requested.
/// It returns a 200 OK response with the body "Hiya y'all!".
#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hiya y'all!")
}

#[get("/hei")]
async fn hei() -> impl Responder {
    HttpResponse::MovedPermanently().body("moved permanently")
}

#[delete("/unauthorized")]
async fn unauthorized() -> impl Responder {
    HttpResponse::Unauthorized().body("unauthorized")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
    cfg.service(hei);
    cfg.service(unauthorized);
}