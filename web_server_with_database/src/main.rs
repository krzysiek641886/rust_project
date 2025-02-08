use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
mod database_handling;
use database_handling::{initialize_db, DB_CONN};
mod orca_slicer_interface;
use orca_slicer_interface::{hello_world_orca_slicer_interface};

// Define a simple handler function
async fn greet() -> impl Responder {
    let db_conn = DB_CONN.lock().unwrap();
    let status = match &*db_conn {
        Some(_) => "Database connection is initialized.",
        None => "Database connection is not initialized.",
    };
    HttpResponse::Ok().json(json!({ "status": status }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the database
    initialize_db("dummy_database.db");
    hello_world_orca_slicer_interface();

    // Start the HTTP server
    HttpServer::new(|| {
        App::new()
            .route("/api/greet", web::get().to(greet)) // Set up a route for the API
            .service(fs::Files::new("/", "./src/frontend").index_file("index.html"))
        // Serve static files
    })
    .bind("127.0.0.1:8080")? // Bind the server to localhost on port 8080
    .run()
    .await
}
