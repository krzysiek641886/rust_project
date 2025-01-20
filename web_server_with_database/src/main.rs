use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod database_handling;
use database_handling::{initialize_db, DB_CONN};

// Define a simple handler function
async fn greet() -> impl Responder {
    let db_conn = DB_CONN.lock().unwrap();
    match &*db_conn {
        Some(_) => HttpResponse::Ok().body("Database connection is initialized."),
        None => HttpResponse::InternalServerError().body("Database connection is not initialized."),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the database
    initialize_db("dummy_database.db");

    // Start the HTTP server
    HttpServer::new(|| {
        App::new().route("/", web::get().to(greet)) // Set up a route for the root path
    })
    .bind("127.0.0.1:8080")? // Bind the server to localhost on port 8080
    .run()
    .await
}
