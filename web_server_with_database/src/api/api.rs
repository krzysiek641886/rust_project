use crate::database_handler;
use actix_multipart::Multipart;
use actix_web::{HttpResponse, Responder};
use database_handler::{add_form_submission_to_db, read_orders_from_db, FormFields};
use futures::StreamExt;
use lazy_static::lazy_static;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

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
        HttpResponse::Ok().body("Application initialized successfully")
    } else {
        HttpResponse::InternalServerError()
            .body("An error occurred during application initialization")
    }
}

pub async fn form_submission_handler(mut payload: Multipart) -> impl Responder {
    let mut form_fields = FormFields {
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
                        "./data_files/{}",
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
    add_form_submission_to_db(form_fields);
    HttpResponse::Ok().body("File uploaded successfully")
}

#[derive(Serialize)]
struct Order {
    name: String,
    email: String,
    copies_nbr: u32,
    file_name: String,
}

pub async fn get_orders_handler() -> impl Responder {
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
