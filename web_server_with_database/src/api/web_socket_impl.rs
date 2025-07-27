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
    pub evaluate_order_cb: fn(&SubmittedOrderData) -> EvaluationResult,
    pub add_evaluation_to_db_cb: fn(&EvaluationResult) -> io::Result<()>,
}

/* PUBLIC TYPES AND VARIABLES */
pub struct PriceEvaluationWebSocketImpl {
    pub evaluate_order_cb: fn(&SubmittedOrderData) -> EvaluationResult,
    pub add_evaluation_to_db_cb: fn(&EvaluationResult) -> io::Result<()>,
}

/* HELPER FUNCTIONS */
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
        "price": format!("{:.2}", eval_result.price),
        "material_type": eval_result.material_type.to_string(),
        "status": "success",
        "message": "Evaluation completed successfully."
    })
    .to_string()
}

/* PRIVATE FUNCTIONS */
impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.my_addr = Some(ctx.address());
    }
}

impl WebSocketSession {
    // Private function to reset the session state
    fn reset_session(&mut self) {
        self.submitted_form = None;
        self.chunks_received = 0;
    }

    fn close_session(&mut self, ctx: &mut ws::WebsocketContext<Self>, error: Option<&str>) {
        self.reset_session();
        let mut close_reason = CloseReason {
            code: ws::CloseCode::Normal,
            description: Some("Session closed".to_string()),
        };
        if let Some(err) = error {
            close_reason.code = ws::CloseCode::Error;
            close_reason.description = Some(format!("Session closed due to an error: {}", err));
        }
        ctx.close(Some(close_reason));
    }

    // Private function for handling the Text payload and parsing it into a SubmittedOrderData struct
    fn handle_text_payload(&mut self, text: String, ctx: &mut ws::WebsocketContext<Self>) {
        match serde_json::from_str::<SubmittedOrderData>(&text) {
            Ok(data) => {
                self.submitted_form = Some(data);
                return;
            }
            _ => self.close_session(
                ctx,
                Some("Failed to parse SubmittedOrderData from text payload"),
            ),
        }
    }

    fn process_stl_model_when_all_chunks_received(
        &mut self,
        ctx: &mut ws::WebsocketContext<Self>,
        form: SubmittedOrderData,
    ) {
        let evaluate_order_function = self.evaluate_order_cb;
        let add_evaluation_to_db_function = self.add_evaluation_to_db_cb;

        let order_evaluation_result = evaluate_order_function(&form);
        if let Err(e) = add_evaluation_to_db_function(&order_evaluation_result) {
            println!("Failed to write evaluation to database. Error: {:?}", e);
            self.close_session(ctx, Some(&format!("Internal database error")));
            return;
        }
        // Serialize the evaluation result to a JSON string
        let json_result = serialize_evaluation_result(order_evaluation_result);
        // Send the evaluation result back to the client
        ctx.text(json_result);
        // Reset the session state after processing
        self.reset_session();
    }

    // Private function for handling the Binary payload and appending the file
    fn handle_binary_payload(
        &mut self,
        bin: Bytes,
        ctx: &mut ws::WebsocketContext<Self>,
        form: SubmittedOrderData,
    ) {
        let filename = &form.file_name;
        let total_chunks = form.nbr_of_chunks;
        if self.chunks_received >= total_chunks {
            self.close_session(
                ctx,
                Some(&format!(
                    "Incorrect number of chunks received {} out of {}",
                    self.chunks_received, total_chunks
                )),
            );
            return;
        }
        if let Ok(chunks_received) = append_the_file(filename, &self.chunks_received, bin) {
            self.chunks_received = chunks_received;
        } else {
            self.close_session(
                ctx,
                Some(&format!(
                    "Internal error while processing the file chunk {}",
                    self.chunks_received + 1
                )),
            );
            return;
        }
        if self.chunks_received == total_chunks {
            self.process_stl_model_when_all_chunks_received(ctx, form);
        }
    }
}

// Implement StreamHandler to handle incoming WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => self.handle_text_payload(text.to_string(), ctx),
            Ok(ws::Message::Binary(bin)) => {
                if self.submitted_form.is_none() {
                    ctx.text("No submitted form data available to process the STL model.");
                    return;
                }
                let form = self.submitted_form.clone().unwrap();
                self.handle_binary_payload(bin, ctx, form);
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
                add_evaluation_to_db_cb: self.add_evaluation_to_db_cb,
                evaluate_order_cb: self.evaluate_order_cb,
            },
            &req,
            stream,
        )
        .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
    }
}

/* TESTS */
