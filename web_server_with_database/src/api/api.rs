/* IMPORTS FROM LIBRARIES */
use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use futures::StreamExt;
use lazy_static::lazy_static;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
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
        websocket_session: Mutex::new(PriceEvaluationWebSocketImpl {}),
    };
}

/* PUBLIC TYPES AND VARIABLES */

/* PRIVATE FUNCTIONS */
fn start_evaluation(form_fields: &SubmittedOrderData) -> bool {
    std::thread::spawn({
        let received_order_cpy = form_fields.clone();
        move || {
            println!(
                "Background evaluation started for: {:?}",
                received_order_cpy
            );
            let eval_result = get_prusa_slicer_evaluation(&received_order_cpy);
            add_evaluation_to_db(&eval_result);
            remove_order_from_db(received_order_cpy);
            send_result_to_client(eval_result);
        }
    });
    return true;
}

/**
 * @brief Sends the evaluation result to the client.
 *
 * This function sends the evaluation result of an order to the client.
 *
 * @param _slicer_evaluation_result Reference to the evaluation result.
 */
fn send_result_to_client(slicer_evaluation_result: EvaluationResult) {
    println!("Send the evaluation result to the client: {:?}", slicer_evaluation_result._price);
    let websocket_session = API_HANDLER_STATE.websocket_session.lock().unwrap();
    websocket_session.send_result_to_websocket(slicer_evaluation_result);
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
    println!("eval_result_websocket_handler");
    let websocket_session = API_HANDLER_STATE.websocket_session.lock().unwrap();
    websocket_session
        .start_web_socket_session(req, stream)
        .await
}

/**
 * @brief Handles form submissions via multipart requests.
 *
 * This function processes form submissions, saves uploaded files, and stores
 * the form data in the database. It also sends updates to the client via WebSocket.
 *
 * @param payload Multipart payload containing the form data and files.
 * @return impl Responder HTTP response indicating the result of the operation.
 */
pub async fn form_submission_handler(mut payload: Multipart) -> impl Responder {
    let mut form_fields = SubmittedOrderData {
        name: None,
        email: None,
        copies_nbr: 0,
        file_name: None,
    };
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(_) => {
                return HttpResponse::InternalServerError().body("Error processing file upload")
            }
        };

        let content_disposition = field.content_disposition();
        match content_disposition.get_name() {
            Some(name) => match name {
                "name" => {
                    form_fields.name = Some(
                        String::from_utf8_lossy(&field.next().await.unwrap().unwrap()).to_string(),
                    );
                    println!("Received name");
                }
                "email" => {
                    form_fields.email = Some(
                        String::from_utf8_lossy(&field.next().await.unwrap().unwrap()).to_string(),
                    );
                    println!("Received email");
                }
                "copies_nbr" => {
                    form_fields.copies_nbr =
                        String::from_utf8_lossy(&field.next().await.unwrap().unwrap())
                            .to_string()
                            .parse()
                            .unwrap();
                    println!("Received copies_nbr");
                }
                "file" => {
                    let filepath: String = format!(
                        "./data_files/received_orders/{}",
                        sanitize_filename::sanitize(&content_disposition.get_filename().unwrap())
                    );

                    let mut f = match File::create(filepath.clone()) {
                        Ok(f) => f,
                        Err(_) => {
                            return HttpResponse::InternalServerError().body("Error creating file")
                        }
                    };

                    while let Some(chunk) = field.next().await {
                        let data = match chunk {
                            Ok(data) => data,
                            Err(_) => {
                                return HttpResponse::InternalServerError()
                                    .body("Error reading file chunk")
                            }
                        };
                        if let Err(_) = f.write_all(&data) {
                            return HttpResponse::InternalServerError()
                                .body("Error writing file chunk");
                        }
                    }
                    form_fields.file_name = Some(filepath);
                    println!("Received file");
                }
                _ => return HttpResponse::BadRequest().body("Unsupported field type"),
            },
            None => return HttpResponse::BadRequest().body("Missing content disposition"),
        }
    }

    if add_form_submission_to_db(&form_fields) == false {
        // Notify the client that the form was successfully submitted
        return HttpResponse::InternalServerError().body("Server database failed");
    }

    if start_evaluation(&form_fields) == false {
        // Notify the client that the form was successfully submitted
        return HttpResponse::InternalServerError().body("Price evaluation failed");
    }

    return HttpResponse::Ok()
        .body("File uploaded successfully. Updates will be sent via WebSocket.");
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
                .map(|order| Order {
                    name: order.name.unwrap_or_default(),
                    email: order.email.unwrap_or_default(),
                    copies_nbr: order.copies_nbr,
                    file_name: order.file_name.unwrap_or_default(),
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
