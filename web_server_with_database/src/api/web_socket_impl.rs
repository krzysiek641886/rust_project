/* IMPORTS FROM LIBRARIES */
use actix::{Actor, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;

/* IMPORTS FROM OTHER MODULES */
use crate::common_utils::global_traits::WebSocketInterfaceImpl;
use crate::common_utils::global_types::EvaluationResult;

/* PRIVATE TYPES AND VARIABLES */
struct WebSocketSession;

/* PUBLIC TYPES AND VARIABLES */
pub struct PriceEvaluationWebSocketImpl {}

/* PRIVATE FUNCTIONS */
impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.text("WebSocket connection established. Updates will be sent here.");
    }
}

// Implement StreamHandler to handle incoming WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Received message: {}", text);
                ctx.text(format!("Echo: {}", text)); // Echo the message back to the client
            }
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg); // Respond to ping with pong
            }
            Ok(ws::Message::Close(reason)) => {
                println!("WebSocket closed: {:?}", reason);
                // ctx.stop(); // Stop the actor
            }
            _ => (),
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
        ws::start(WebSocketSession {}, &req, stream)
            .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
    }

    fn send_result_to_websocket(&self, _slicer_evaluation_result: EvaluationResult) {
        println!("send_result_to_websocket");
    }
}

/* TESTS */
