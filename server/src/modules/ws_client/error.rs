use thiserror::Error;
use crate::modules::ws_client::models::ws_messages::OutgoingMessage;

#[derive(Debug, Error)]
pub enum WsError {
    #[error("{0}")]
    TungsteniteError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Outgoing error (omitted)")]
    OutgoingError(OutgoingMessage),

    #[error("{0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Socket closed")]
    SocketClosed,
}

impl From<OutgoingMessage> for WsError {
    fn from(value: OutgoingMessage) -> Self {
        WsError::OutgoingError(value)
    }
}
