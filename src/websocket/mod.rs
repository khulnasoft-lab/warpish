//! Real-time Communication Support
//!
//! This module provides real-time communication support using WebSockets.

use futures_util::{StreamExt, SinkExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebSocketError {
    #[error("Failed to connect to WebSocket: {0}")]
    ConnectFailed(tokio_tungstenite::tungstenite::Error),
    #[error("WebSocket error: {0}")]
    SocketError(tokio_tungstenite::tungstenite::Error),
}

pub struct WebSocketClient {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WebSocketClient {
    pub async fn connect(url: &str) -> Result<Self, WebSocketError> {
        let (socket, _response) = connect_async(url)
            .await
            .map_err(WebSocketError::ConnectFailed)?;
        Ok(Self { socket })
    }

    pub async fn send(&mut self, message: &str) -> Result<(), WebSocketError> {
        self.socket
            .send(Message::Text(message.to_string()))
            .await
            .map_err(WebSocketError::SocketError)
    }

    pub async fn recv(&mut self) -> Option<Result<Message, WebSocketError>> {
        self.socket.next().await.map(|res| res.map_err(WebSocketError::SocketError))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;
    use tokio_tungstenite::accept_async;

    async fn echo_server(mut stream: WebSocketStream<TcpStream>) {
        while let Some(msg) = stream.next().await {
            let msg = msg.unwrap();
            if msg.is_text() || msg.is_binary() {
                stream.send(msg).await.unwrap();
            }
        }
    }

    #[tokio::test]
    async fn test_websocket_client() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            let (stream, _request) = listener.accept().await.unwrap();
            let ws_stream = accept_async(stream).await.unwrap();
            echo_server(ws_stream).await;
        });

        let mut client = WebSocketClient::connect(&format!("ws://{}", addr))
            .await
            .unwrap();

        client.send("hello").await.unwrap();

        let msg = client.recv().await.unwrap().unwrap();
        assert_eq!(msg.to_text().unwrap(), "hello");
    }
}
