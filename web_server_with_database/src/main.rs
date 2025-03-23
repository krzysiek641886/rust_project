use actix_files as fs;
use actix_web::{web, App, HttpServer};
use clap::Parser;
mod database_handler;
use database_handler::{
    add_evaluation_to_db, get_pending_order, initialize_db, remove_order_from_db,
};
use lazy_static::lazy_static;
mod orca_slicer_interface;
use orca_slicer_interface::{
    get_orca_slicer_evaluation, initialize_orca_slicer_if, EvaluationResult,
};
use std::sync::Mutex;
use tokio::time::{interval, Duration};
mod api;
use api::{
    app_init_status_handler, form_submission_handler, get_orders_handler, initialize_api_handler,
    send_result_to_client,
};

/// Command-line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Database name
    #[clap(
        short = 'd',
        long = "db-name",
        default_value = "default_db_name.db",
        help = "Database name for storing orders"
    )]
    db_name: String,
    #[clap(short = 'w', long = "ws-path", help = "Path to the server directory")]
    ws_path: String,
    #[clap(
        short = 'o',
        long = "orca-slicer-path",
        help = "Path to Orca Slicer executable"
    )]
    orca_path: String,
    #[clap(short = 's', long = "system", help = "Path to Orca Slicer executable")]
    system: String,
}

struct State {
    ws_path: Mutex<String>,
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
    initialize_api_handler(true);
}

async fn process_orders_periodically(interval_seconds: u64) {
    let interval_duration = Duration::from_secs(interval_seconds); // Check every 60 seconds
    let mut interval: tokio::time::Interval = interval(interval_duration);

    loop {
        println!("Checking for pending orders...");
        match get_pending_order() {
            Some(order) => {
                let slicer_evaluation_result: EvaluationResult = get_orca_slicer_evaluation(&order);
                send_result_to_client(&slicer_evaluation_result);
                add_evaluation_to_db(slicer_evaluation_result);
                remove_order_from_db(order);
            }
            None => {
                interval.tick().await;
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    initialize_modules_with_cmd_arguments(args);

    // Spawn the background task for processing orders
    tokio::spawn(process_orders_periodically(60));

    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .route("/api/backendstatus", web::get().to(app_init_status_handler))
            .route("/api/upload", web::post().to(form_submission_handler)) // Add API route for file upload
            .route("/api/orders", web::get().to(get_orders_handler)) // Add API route for file upload
            // The index page has to be initialized after API endpoints
            .service(fs::Files::new("/", "./src/frontend").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")? // Bind the server to localhost on port 8080
    .run()
    .await
}
