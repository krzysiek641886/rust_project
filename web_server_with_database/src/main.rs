use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rusqlite::Connection;
use std::sync::Mutex;

// Define a global mutable variable protected by a Mutex
static DB_CONN: Mutex<Option<Connection>> = Mutex::new(None);

// Define a simple handler function
async fn greet() -> impl Responder {
    let db_conn = DB_CONN.lock().unwrap();
    match &*db_conn {
        Some(_) => HttpResponse::Ok().body("Database connection is initialized."),
        None => HttpResponse::InternalServerError().body("Database connection is not initialized."),
    }
}
// Function to initialize the database connection
fn initialize_db() {
    let conn = Connection::open("dummy_database.db").expect("Failed to open database");
    conn.execute(
        "create table if not exists dummy_table (
            id integer primary key,
            name text not null unique
            )",
        [],
    )
    .expect("Failed to create dummy_table table");
    let mut db_conn = DB_CONN.lock().unwrap();
    *db_conn = Some(conn);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the database
    initialize_db();

    // Start the HTTP server
    HttpServer::new(|| {
        App::new().route("/", web::get().to(greet)) // Set up a route for the root path
    })
    .bind("127.0.0.1:8080")? // Bind the server to localhost on port 8080
    .run()
    .await
}
