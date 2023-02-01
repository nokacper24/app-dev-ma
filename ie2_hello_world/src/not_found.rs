use actix_web::{HttpResponse, Responder};

/// This function is called when no other route matches the request.
/// It returns a 404 Not Found response.
pub async fn not_found() -> impl Responder {
    let body = r"
        <html>
            <head>
                <title>404 Not Found</title>
            </head>
            <body>
                <h1>404 Not Found</h1>
                <p>Could not find requested page.</p>
            </body>
        </html>";

    HttpResponse::NotFound().body(body)
}