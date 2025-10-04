/* MODULES USED BY THE PROJECT */
mod api;
mod common_utils; // Add this line to declare the module
mod database_handler;
mod prusa_slicer_interface;

/* IMPORTS FROM LIBRARIES */
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use lazy_static::lazy_static;
use std::sync::Mutex;

/* IMPORTS FROM OTHER MODULES */
use api::{
    app_init_status_handler, eval_result_websocket_handler, get_completed_orders_handler,
    get_orders_handler, initialize_api_handler, modify_completed_order_handler,
    modify_order_handler,
};
use database_handler::initialize_db;
use prusa_slicer_interface::initialize_prusa_slicer_if;

/* PRIVATE TYPES AND VARIABLES */
/// Command-line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(
        short = 'p',
        long = "app-params",
        help = "Path to the price parameters file",
        default_value = "data_files/print_price_evaluator_config.json"
    )]
    app_params: String,
}

struct State {
    ws_path: Mutex<String>,
}
lazy_static! {
    static ref MAIN_STATE: State = State {
        ws_path: Mutex::new(String::from("")),
    };
}

/* PUBLIC TYPES AND VARIABLES */

/* PRIVATE FUNCTIONS */

/**
 * @brief Initializes modules with command-line arguments.
 *
 * This function initializes the database, Prusa Slicer interface, and API handler
 * using the provided command-line arguments. It also updates the global state
 * with the workspace path.
 *
 * @param args Command-line arguments parsed into an Args struct.
 */
fn initialize_modules_with_cmd_arguments(_args: Args) {
    // This consumes the args struct and initializes the global state. No other use of args is allowed after this point.
    let app_db_name = "data_files/price_evaluator_database.db";
    let app_prusa_path = "/Applications/PrusaSlicer.app/Contents/MacOS/PrusaSlicer";
    let app_app_params = "data_files/print_price_evaluator_config.json";
    let app_ws_path = "/Users/krzysztofmroz/Projects/rust_project/web_server_with_database";
    initialize_db(&app_db_name);
    initialize_prusa_slicer_if(&app_ws_path, &app_prusa_path, &app_app_params)
        .expect("Failed to initialize Prusa Slicer interface");
    let mut ws_path_lock = MAIN_STATE.ws_path.lock().unwrap();
    *ws_path_lock = app_ws_path.to_string();
    initialize_api_handler(true);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    initialize_modules_with_cmd_arguments(args);

    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .route("/api/backendstatus", web::get().to(app_init_status_handler))
            .route("/api/orders", web::get().to(get_orders_handler))
            .route("/api/orders/modify", web::put().to(modify_order_handler))
            .route(
                "/api/completed_orders",
                web::get().to(get_completed_orders_handler),
            )
            .route(
                "/api/completed_orders/modify",
                web::put().to(modify_completed_order_handler),
            )
            .route(
                "/api/websocket_evaluation",
                web::get().to(eval_result_websocket_handler),
            ) // Add WebSocket route
            // The index page has to be initialized after API endpoints
            .service(fs::Files::new("/", "./src/frontend").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")? // Bind the server to localhost on port 8080
    .run()
    .await
}

/* PUBLIC FUNCTIONS */

/* TESTS */
