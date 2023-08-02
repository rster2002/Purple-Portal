use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, WebSocketStream};
use crate::modules::ws_client::error::WsError;

pub mod models;
pub mod error;

pub struct WsClient {
    ws_stream: WebSocketStream<TcpStream>,
}

impl WsClient {
    pub async fn accept(stream: TcpStream) -> Result<Self, WsError> {
        let ws_stream = accept_async(stream)
            .await?;

        Ok(Self {
            ws_stream,
        })
    }

    pub async fn send(&self) -> Result<(), WsError> {
        todo!()
    }

    pub async fn receive(&mut self) {
        todo!()
    }
}
