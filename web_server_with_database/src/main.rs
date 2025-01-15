use actix_web::{web, App, HttpServer, Responder, HttpResponse};

// Define a simple handler function
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust server!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the HTTP server
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet)) // Set up a route for the root path
    })
    .bind("127.0.0.1:8080")? // Bind the server to localhost on port 8080
    .run()
    .await
}