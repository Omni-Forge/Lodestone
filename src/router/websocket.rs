use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};

pub struct WebSocketHandler;

impl WebSocketHandler {
    pub async fn handle_upgrade(ws: WebSocketUpgrade) -> Response {
        ws.on_upgrade(Self::handle_socket)
    }

    async fn handle_socket(mut socket: WebSocket) {
        while let Some(msg) = socket.recv().await {
            if let Ok(msg) = msg {
                if socket.send(msg).await.is_err() {
                    break;
                }
            } else {
                break;
            }
        }
    }
}