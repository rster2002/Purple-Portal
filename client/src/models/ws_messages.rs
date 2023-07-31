use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum WsClientIncoming {
    AuthenticationFailed,
    AuthenticationSuccess,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum WsClientOutgoing {
    UnprocessableContent,
    Authenticate {
        password: String,
    },
}
