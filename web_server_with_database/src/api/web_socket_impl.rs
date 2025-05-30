/* IMPORTS FROM LIBRARIES */
use actix::AsyncContext;
use actix::{Actor, Addr, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::io;
use bytes::Bytes;

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_traits::WebSocketInterfaceImpl;
use crate::common_utils::global_types::{EvaluationResult, SubmittedOrderData};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

/* PRIVATE TYPES AND VARIABLES */
struct WebSocketSession {
    pub my_addr: Option<Addr<WebSocketSession>>,
    submitted_form: Option<SubmittedOrderData>,
    chunks_received: u32,
    pub evaluate_order_cb: fn(&SubmittedOrderData) -> EvaluationResult,
}

/* PUBLIC TYPES AND VARIABLES */
pub struct PriceEvaluationWebSocketImpl {
    pub evaluate_order_cb: fn(&SubmittedOrderData) -> EvaluationResult,

}

/* PRIVATE FUNCTIONS */
fn append_the_file(
    filename: &String,
    total_chunks: u32,
    chunks_received: u32,
    bin: Bytes,
) -> io::Result<u32> {
    let file_path = Path::new("data_files/received_orders/").join(filename);
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)?;
    file.write_all(&bin)?;
    println!(
        "Appended chunk {}/{} to file {}",
        chunks_received + 1,
        total_chunks,
        filename
    );
    Ok(chunks_received + 1)
}

fn serialize_evaluation_result(_eval_result: EvaluationResult) -> String {
    // Serialize the EvaluationResult to a JSON string
    String::from("{\"status\": \"success\", \"message\": \"Evaluation completed successfully.\"}")
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.text("WebSocket connection established. Updates will be sent here.");
        self.my_addr = Some(ctx.address());
    }
}

impl WebSocketSession {
    // Private helper function to reset the session state
    fn reset_session(&mut self) {
        self.submitted_form = None;
        self.chunks_received = 0;
    }
}

// Implement StreamHandler to handle incoming WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("Inside StreamHandler msg");
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Received message {}", text);
                self.submitted_form = {
                    // Parse the text message into a SubmittedOrderData struct
                    match serde_json::from_str::<SubmittedOrderData>(&text) {
                        Ok(data) => Some(data),
                        Err(e) => {
                            println!("Failed to parse SubmittedOrderData: {}", e);
                            None
                        }
                    }
                };
                ctx.text(format!("Echo: {}", text)); // Echo the message back to the client
            }
            Ok(ws::Message::Binary(bin)) => {
                // Handle file upload (binary data)
                println!("Received binary data of length: {}", bin.len());
                // Here you could process the file, save it, etc.
                ctx.text(format!("Received file of {} bytes", bin.len()));

                if let Some(ref form) = self.submitted_form {
                    let filename = &form.file_name;
                    let total_chunks = form.nbr_of_chunks;
                    if self.chunks_received >= total_chunks {
                        panic!("Incorrect number of chunks. TBA handled");
                    }
                    match append_the_file(filename, total_chunks, self.chunks_received, bin) {
                        Ok(chunks_received) => {
                            self.chunks_received = chunks_received;
                        }
                        Err(e) => {
                            println!("Failed to append file: {}", e);
                            ctx.text(format!("Error uploading the file: {}", e));
                            return;
                        }
                    }

                    if self.chunks_received == total_chunks {
                        println!("Upload complete for file: {}", filename);
                        let result = (self.evaluate_order_cb)(form);
                        // Serialize the evaluation result to a JSON string
                        let json_result = serialize_evaluation_result(result);
                        // Send the evaluation result back to the client
                        ctx.text(json_result);
                        // Reset the session state after processing
                        self.reset_session();
                    }
                } else {
                    println!("No submitted form data available to get filename and total_chunks.");
                    return;
                }
            }
            _ => {
                println!("Received unsupported WebSocket message type");
                ctx.text("Unsupported message type. Only text and binary messages are supported.");
            }
        }
    }
}

/* PUBLIC FUNCTIONS */
impl WebSocketInterfaceImpl for PriceEvaluationWebSocketImpl {
    async fn start_web_socket_session(
        &self,
        req: HttpRequest,
        stream: web::Payload,
    ) -> HttpResponse {
        println!("start_web_socket_session");
        ws::start(
            WebSocketSession {
                my_addr: None,
                submitted_form: None,
                chunks_received: 0,
                evaluate_order_cb: self.evaluate_order_cb,
            },
            &req,
            stream,
        )
        .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
    }
}

/* TESTS */
