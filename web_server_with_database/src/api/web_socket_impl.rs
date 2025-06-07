/* IMPORTS FROM LIBRARIES */
use actix::AsyncContext;
use actix::{Actor, Addr, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws::{self, CloseReason};
use bytes::Bytes;
use std::io;

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
    pub add_form_submission_to_db_cb: fn(&SubmittedOrderData) -> bool,
    pub evaluate_order_cb: fn(&SubmittedOrderData) -> EvaluationResult,
}

/* PUBLIC TYPES AND VARIABLES */
pub struct PriceEvaluationWebSocketImpl {
    pub add_form_submission_to_db_cb: fn(&SubmittedOrderData) -> bool,
    pub evaluate_order_cb: fn(&SubmittedOrderData) -> EvaluationResult,
}

/* PRIVATE FUNCTIONS */
fn append_the_file(filename: &String, chunks_received: &u32, bin: Bytes) -> io::Result<u32> {
    let file_path = Path::new("data_files/received_orders/").join(filename);
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    if (*chunks_received == 0) && file_path.exists() {
        //Remove the file if it exists, to start fresh
        std::fs::remove_file(&file_path)?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)?;
    file.write_all(&bin)?;
    Ok(chunks_received + 1)
}

fn serialize_evaluation_result(eval_result: EvaluationResult) -> String {
    // Serialize the EvaluationResult to a JSON string
    serde_json::json!({
        "type": "evaluation_result",
        "name": eval_result.name,
        "email": eval_result.email,
        "copies_nbr": eval_result.copies_nbr,
        "file_name": eval_result.file_name,
        "price": eval_result.price,
        "status": "success",
        "message": "Evaluation completed successfully."
    })
    .to_string()
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
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
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Parse the text message into a SubmittedOrderData struct
                match serde_json::from_str::<SubmittedOrderData>(&text) {
                    Ok(data) => {
                        if (self.add_form_submission_to_db_cb)(&data) {
                            self.submitted_form = Some(data);
                            return;
                        }
                    }
                    _ => {}
                }
                self.reset_session();
                let close_reason = CloseReason {
                    code: ws::CloseCode::Invalid,
                    description: None,
                };
                ctx.close(Some(close_reason));
            }
            Ok(ws::Message::Binary(bin)) => {
                if let Some(ref form) = self.submitted_form {
                    let filename = &form.file_name;
                    let total_chunks = form.nbr_of_chunks;
                    if self.chunks_received >= total_chunks {
                        panic!("Incorrect number of chunks. TBA handling");
                    }
                    match append_the_file(filename, &self.chunks_received, bin) {
                        Ok(chunks_received) => {
                            self.chunks_received = chunks_received;
                        }
                        Err(_e) => {
                            self.reset_session();
                            let close_reason = CloseReason {
                                code: ws::CloseCode::Error,
                                description: None,
                            };
                            ctx.close(Some(close_reason));
                            return;
                        }
                    }
                    if self.chunks_received == total_chunks {
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
        ws::start(
            WebSocketSession {
                my_addr: None,
                submitted_form: None,
                chunks_received: 0,
                add_form_submission_to_db_cb: self.add_form_submission_to_db_cb,
                evaluate_order_cb: self.evaluate_order_cb,
            },
            &req,
            stream,
        )
        .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
    }
}

/* TESTS */
