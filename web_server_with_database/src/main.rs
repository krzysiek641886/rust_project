use actix_files as fs;
use actix_web::{App, HttpServer};
use clap::Parser;
mod database_handler;
use database_handler::initialize_db;
mod orca_slicer_interface;
use orca_slicer_interface::initialize_orca_slicer_if;
use std::sync::Mutex;
use lazy_static::lazy_static;

/// Command-line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Database name
    #[clap(short='d', long="db-name", default_value = "default_db_name.db", help = "Database name for storing orders")]
    db_name: String,
    #[clap(short='w', long="ws-path", help = "Path to the server directory")]
    ws_path: String,
    #[clap(short='o', long="orca-slicer-path", help = "Path to Orca Slicer executable")]
    orca_path: String,
    #[clap(short='s', long="system", help = "Path to Orca Slicer executable")]
    system: String,
}

struct State {
    ws_path: Mutex<String>
}
lazy_static! {
    static ref MAIN_STATE: State = State {
        ws_path: Mutex::new(String::from("")),
};
}

fn initialize_modules_with_cmd_arguments(args: Args) {
    // This consumes the args struct and initializes the global state. No other use of args is allowed after this point.
    initialize_db(&args.db_name);
    initialize_orca_slicer_if(&args.system, &args.orca_path);
    let mut ws_path_lock = MAIN_STATE.ws_path.lock().unwrap();
    *ws_path_lock = args.ws_path.to_string();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Parse command-line arguments
    let args = Args::parse();
    initialize_modules_with_cmd_arguments(args);

    // Start the HTTP server
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/", "./src/frontend").index_file("index.html")) // Serve static files
    })
    .bind("127.0.0.1:8080")? // Bind the server to localhost on port 8080
    .run()
    .await
}
