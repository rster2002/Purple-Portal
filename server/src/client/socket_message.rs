use serde::{Deserialize, Serialize};
use crate::models::client_info::ClientInfo;
use crate::models::remote_op_log::RemoteOpLog;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum IncomingSocketMessage {
    UnprocessableContent,
    Authenticate {
        client_info: ClientInfo,
        password: String,
    },

    Sync(RemoteOpLog),
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum OutgoingSocketMessage {
    AuthenticationFailed,
    AuthenticationSuccess,

    IncorrectTime,

    #[serde(rename = "error")]
    ClientError(ErrorMessage),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorMessage {
    pub error: String,
    pub message: String,
}
