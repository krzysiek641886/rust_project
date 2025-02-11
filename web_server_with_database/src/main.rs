use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use serde_json::json;
mod database_handling;
use database_handling::{initialize_db, DB_CONN};
mod orca_slicer_interface;
use orca_slicer_interface::hello_world_orca_slicer_interface;

/// Command-line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Database name
    #[clap(short='d', long="db-name", default_value = "default_db_name.db")]
    db_name: String,
    #[clap(short='w', long="ws-path", help = "Path to the server directory")]
    ws_path: String,
    #[clap(short='o', long="orca-slicer-path", help = "Path to Orca Slicer executable")]
    orca_path: String,
}

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
    // Parse command-line arguments
    let args = Args::parse();

    // Initialize the database
    initialize_db(&args.db_name);
    hello_world_orca_slicer_interface();

    // Start the HTTP server
    HttpServer::new(|| {
        App::new()
            .route("/api/greet", web::get().to(greet)) // Set up a route for the API
            .service(fs::Files::new("/", "./src/frontend").index_file("index.html")) // Serve static files
    })
    .bind("127.0.0.1:8080")? // Bind the server to localhost on port 8080
    .run()
    .await
}
