/* IMPORTS FROM LIBRARIES */
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use lazy_static::lazy_static;
use serde::Serialize;
use std::sync::Mutex;

/* IMPORTS FROM OTHER MODULES */
use crate::api::web_socket_impl::PriceEvaluationWebSocketImpl;
use crate::common_utils::global_traits::WebSocketInterfaceImpl;
use crate::common_utils::global_types::{EvaluationResult, PrintMaterialType};
use crate::database_handler::{
    add_evaluation_to_db, modify_order_in_database, read_completed_orders_from_db,
    read_orders_from_db,
};
use crate::prusa_slicer_interface::get_prusa_slicer_evaluation;
use serde::Deserialize;

/* PRIVATE TYPES AND VARIABLES */
struct State {
    app_init_status: Mutex<bool>,
    websocket_session: Mutex<PriceEvaluationWebSocketImpl>,
}

lazy_static! {
    static ref API_HANDLER_STATE: State = State {
        app_init_status: Mutex::new(false),
        websocket_session: Mutex::new(PriceEvaluationWebSocketImpl {
            add_evaluation_to_db_cb: add_evaluation_to_db,
            evaluate_order_cb: get_prusa_slicer_evaluation,
        }),
    };
}

/* PUBLIC TYPES AND VARIABLES */

/* PRIVATE FUNCTIONS */

/* PUBLIC FUNCTIONS */
/**
 * @brief Initializes the API handler.
 *
 * This function sets the application initialization status in the global state.
 *
 * @param app_init_status Boolean indicating whether the application is initialized.
 */
pub fn initialize_api_handler(app_init_status: bool) {
    let mut app_init_status_lock = API_HANDLER_STATE.app_init_status.lock().unwrap();
    *app_init_status_lock = app_init_status;
}

/**
 * @brief Handles the application initialization status API endpoint.
 *
 * This function returns an HTTP response indicating whether the application
 * was initialized successfully.
 *
 * @return impl Responder HTTP response with the initialization status.
 */
pub async fn app_init_status_handler() -> impl Responder {
    if *API_HANDLER_STATE.app_init_status.lock().unwrap() {
        HttpResponse::Ok().body("Application initialized successfully")
    } else {
        HttpResponse::InternalServerError()
            .body("An error occurred during application initialization")
    }
}

/**
 * @brief Handles WebSocket connections for updates.
 *
 * This function upgrades the HTTP connection to a WebSocket connection
 * and sends updates to the client.
 *
 * @param req HTTP request.
 * @param stream WebSocket stream.
 * @return HttpResponse WebSocket response.
 */
pub async fn eval_result_websocket_handler(req: HttpRequest, stream: web::Payload) -> HttpResponse {
    let websocket_session = API_HANDLER_STATE.websocket_session.lock().unwrap();
    websocket_session
        .start_web_socket_session(req, stream)
        .await
}

/**
 * @brief Handles the API endpoint to retrieve orders.
 *
 * This function retrieves all orders from the database and returns them as a JSON response.
 *
 * @return impl Responder HTTP response containing the orders in JSON format.
 */
pub async fn get_orders_handler() -> impl Responder {
    #[derive(Serialize)]
    struct Order {
        date: String,
        name: String,
        email: String,
        copies_nbr: u32,
        file_name: String,
        price: f64,
        material_type: PrintMaterialType,
        print_type: String,
        status: String,
    }
    match read_orders_from_db() {
        Ok(orders) => {
            let orders_json: Vec<Order> = orders
                .into_iter()
                .map(|order: EvaluationResult| Order {
                    date: order.date.to_string(),
                    name: order.name,
                    email: order.email,
                    copies_nbr: order.copies_nbr,
                    file_name: order.file_name,
                    price: order.price,
                    material_type: order.material_type,
                    print_type: order.print_type.to_string(),
                    status: order.status.to_string(),
                })
                .collect();
            HttpResponse::Ok().json(orders_json)
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to retrieve orders: {}", e))
        }
    }
}

pub async fn get_completed_orders_handler() -> impl Responder {
    #[derive(Serialize)]
    struct Order {
        date: String,
        name: String,
        email: String,
        copies_nbr: u32,
        file_name: String,
        price: f64,
        material_type: PrintMaterialType,
        print_type: String,
        status: String,
    }
    match read_completed_orders_from_db() {
        Ok(orders) => {
            let orders_json: Vec<Order> = orders
                .into_iter()
                .map(|order: EvaluationResult| Order {
                    date: order.date.to_string(),
                    name: order.name,
                    email: order.email,
                    copies_nbr: order.copies_nbr,
                    file_name: order.file_name,
                    price: order.price,
                    material_type: order.material_type,
                    print_type: order.print_type.to_string(),
                    status: order.status.to_string(),
                })
                .collect();
            HttpResponse::Ok().json(orders_json)
        }
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Failed to retrieve completed orders: {}", e)),
    }
}

#[derive(Deserialize)]
pub struct OrderModification {
    datetime: String,
    new_status: String,
    // Add any other modifiable fields
}

pub async fn modify_order_handler(payload: web::Json<OrderModification>) -> impl Responder {
    // Process the modification
    match modify_order_in_database(&payload.datetime, &payload.new_status) {
        Ok(_) => {
            // For now, just return success (replace with actual implementation)
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "Order modified successfully",
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to modify order: {}", e))
        }
    }
}

/* TESTS */
