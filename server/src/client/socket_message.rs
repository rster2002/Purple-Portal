use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum ReceivedSocketMessage {
    Authenticate {
        password: String,
    },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum SendSocketMessage {
    AuthenticationFailed,
    AuthenticationSuccess,
}
