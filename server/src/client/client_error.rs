use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Client failed to authenticate")]
    AuthenticationFailed,

    #[error("WebSocket error: {0}")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("{0}")]
    JsonError(#[from] serde_json::Error),
}

impl ClientError {
    pub fn type_string(&self) -> String {
        match self {
            ClientError::AuthenticationFailed => "authenticationFailed",
            _ => "internal",
        }
        .to_string()
    }
}
