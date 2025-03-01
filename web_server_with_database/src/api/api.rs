use actix_web::{Responder, HttpResponse};
use std::sync::Mutex;
use lazy_static::lazy_static;

struct State {
    app_init_status: Mutex<bool>,
}

lazy_static! {
    static ref API_HANDLER_STATE: State = State {
        app_init_status: Mutex::new(false),
    };
}

// Function to initialize the database connection
pub fn initialize_api_handler(app_init_status: bool) {
    let mut app_init_status_lock = API_HANDLER_STATE.app_init_status.lock().unwrap();
    *app_init_status_lock = app_init_status;
}

// API handler function
pub async fn app_init_status_handler() -> impl Responder {
    if *API_HANDLER_STATE.app_init_status.lock().unwrap() {
        return HttpResponse::Ok().body("Application initialized successfully");
    } else {
        return HttpResponse::InternalServerError().body("An error occurred during application initialization");
    }
}