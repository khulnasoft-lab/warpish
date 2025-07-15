//! Real-time Communication Support
//!
//! This module provides real-time communication support using WebSockets.

use anyhow::Result;
use futures_util::{StreamExt, SinkExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream, tungstenite::Error as WsError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebSocketError {
    #[error("Socket error: {0}")]
    SocketError(#[from] WsError), // FIX: Implemented From trait
}

pub struct WebSocketClient {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WebSocketClient {
    pub async fn new(url: &str) -> Result<Self, WebSocketError> {
        let (socket, _) = connect_async(url).await?;
        Ok(Self { socket })
    }

    pub async fn send_message(&mut self, message: &str) -> Result<(), WebSocketError> {
        self.socket
            .send(Message::Text(message.to_string()))
            .await?;
        Ok(())
    }

    pub async fn receive_message(&mut self) -> Result<Option<String>, WebSocketError> {
        match self.socket.next().await {
            Some(Ok(Message::Text(text))) => Ok(Some(text)),
            Some(Ok(_)) => Ok(None), // Ignore other message types for now
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
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

        let mut client = WebSocketClient::new(&format!("ws://{}", addr))
            .await
            .unwrap();

        client.send_message("hello").await.unwrap();

        let msg = client.receive_message().await.unwrap().unwrap();
        assert_eq!(msg, "hello");
    }
}
