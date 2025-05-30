/* IMPORTS FROM LIBRARIES */
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use lazy_static::lazy_static;
use serde::Serialize;
use std::sync::Mutex;

/* IMPORTS FROM OTHER MODULES */
use crate::api::web_socket_impl::PriceEvaluationWebSocketImpl;
use crate::common_utils::global_traits::WebSocketInterfaceImpl;
use crate::common_utils::global_types::{EvaluationResult, SubmittedOrderData};
use crate::database_handler::{
    add_evaluation_to_db, add_form_submission_to_db, read_orders_from_db, remove_order_from_db,
};
use crate::prusa_slicer_interface::get_prusa_slicer_evaluation;

/* PRIVATE TYPES AND VARIABLES */
struct State {
    app_init_status: Mutex<bool>,
    websocket_session: Mutex<PriceEvaluationWebSocketImpl>,
}

lazy_static! {
    static ref API_HANDLER_STATE: State = State {
        app_init_status: Mutex::new(false),
        websocket_session: Mutex::new(PriceEvaluationWebSocketImpl {
            add_form_submission_to_db_cb: add_form_submission_to_db,
            evaluate_order_cb: evaluate_order,
        }),
    };
}

/* PUBLIC TYPES AND VARIABLES */

/* PRIVATE FUNCTIONS */
fn evaluate_order(form_fields: &SubmittedOrderData) -> EvaluationResult {
    let eval_result = get_prusa_slicer_evaluation(&form_fields);
    add_evaluation_to_db(&eval_result);
    remove_order_from_db(&form_fields);
    return eval_result;
}

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
        name: String,
        email: String,
        copies_nbr: u32,
        file_name: String,
    }
    match read_orders_from_db() {
        Ok(orders) => {
            let orders_json: Vec<Order> = orders
                .into_iter()
                .map(|order: SubmittedOrderData| Order {
                    name: order.name,
                    email: order.email,
                    copies_nbr: order.copies_nbr,
                    file_name: order.file_name,
                })
                .collect();
            HttpResponse::Ok().json(orders_json)
        }
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to retrieve orders: {}", e))
        }
    }
}

/* TESTS */
