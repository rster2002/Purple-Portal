use thiserror::Error;

#[derive(Debug, Error)]
pub enum WsError {
    #[error("{0}")]
    TungsteniteError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("Socket closed")]
    SocketClosed,
}
