use serde::{Deserialize, Serialize};
use crate::modules::signalling_manager::models::remote_op_log::RemoteOpLog;
use crate::modules::ws_client::models::client_info::ClientInfo;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum IncomingMessage {
    AuthenticationFailed,
    AuthenticationSuccess,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum OutgoingMessage {
    UnprocessableContent,
    Authenticate {
        client_info: ClientInfo,
        password: String,
    },
    Sync(RemoteOpLog),
}
