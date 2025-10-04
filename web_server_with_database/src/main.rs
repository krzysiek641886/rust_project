/* MODULES USED BY THE PROJECT */
mod api;
mod common_utils; // Add this line to declare the module
mod database_handler;
mod prusa_slicer_interface;

/* IMPORTS FROM LIBRARIES */
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use clap::Parser;

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

/* PUBLIC TYPES AND VARIABLES */

/* PRIVATE FUNCTIONS */
fn get_current_working_directory() -> String {
    std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

/**
 * @brief Initializes modules with command-line arguments.
 *
 * This function initializes the database, Prusa Slicer interface, and API handler
 * using the provided command-line arguments. It also updates the global state
 * with the workspace path.
 *
 * @param args Command-line arguments parsed into an Args struct.
 */
fn initialize_modules_with_cmd_arguments(args: Args) {
    let print_price_evaluator_config_path = &args.app_params;
    let db_name = "data_files/price_evaluator_database.db"; // Hardcoded for now, might be part of config later
    let ws_path = get_current_working_directory();
    initialize_db(&db_name);
    initialize_prusa_slicer_if(&ws_path, &print_price_evaluator_config_path)
        .expect("Failed to initialize Prusa Slicer interface");
    initialize_api_handler(true);
}

/* PUBLIC FUNCTIONS */
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

/* TESTS */
