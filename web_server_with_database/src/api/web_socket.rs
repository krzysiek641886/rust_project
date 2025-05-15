use actix::{Actor, StreamHandler};
use actix_web::{HttpRequest, HttpResponse, web};
use actix_web_actors::ws;

// WebSocket session struct
pub struct WebSocketSession;

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

// WebSocket Interface
pub async fn start_websocket(req: HttpRequest, stream: web::Payload) -> HttpResponse {
    ws::start(WebSocketSession {}, &req, stream).unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
}
