use diamond_types::LocalVersion;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum IncomingSocketMessage {
    Authenticate {
        password: String,
    },

    Sync {
        last_sync: LocalVersion,
        op_log: Vec<u8>,
    },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum OutgoingSocketMessage {
    AuthenticationFailed,
    AuthenticationSuccess,

    #[serde(rename = "error")]
    ClientError(ErrorMessage),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage {
    pub error: String,
    pub message: String,
}
