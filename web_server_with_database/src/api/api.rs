use actix_web::{/*web,*/ HttpResponse, Responder};
use lazy_static::lazy_static;
use std::sync::Mutex;
// use actix_multipart::Multipart;
// use futures::StreamExt;
// use std::io::Write;
// use std::fs::File;

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

pub async fn upload_file_handler(/*mut payload: Multipart*/) -> impl Responder {
    // while let Some(item) = payload.next().await {
    //     let mut field = match item {
    //         Ok(field) => field,
    //         Err(_) => return HttpResponse::InternalServerError().body("Error processing file upload"),
    //     };

    //     let content_disposition = field.content_disposition().unwrap();
    //     let filename = content_disposition.get_filename().unwrap();
    //     let filepath = format!("./uploads/{}", sanitize_filename::sanitize(&filename));

    //     let mut f = match File::create(filepath) {
    //         Ok(f) => f,
    //         Err(_) => return HttpResponse::InternalServerError().body("Error creating file"),
    //     };

    //     while let Some(chunk) = field.next().await {
    //         let data = match chunk {
    //             Ok(data) => data,
    //             Err(_) => return HttpResponse::InternalServerError().body("Error reading file chunk"),
    //         };
    //         if let Err(_) = f.write_all(&data) {
    //             return HttpResponse::InternalServerError().body("Error writing file chunk");
    //         }
    //     }
    // }

    // HttpResponse::Ok().body("File uploaded successfully")
    return HttpResponse::InternalServerError().body("File upload not yet implemented");
}
