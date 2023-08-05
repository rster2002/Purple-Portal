use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::modules::signalling_manager::models::remote_op_log::RemoteOpLog;
use crate::modules::ws_client::models::client_info::ClientInfo;

/// An incoming message from the client (send by the client TO the server)
#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum IncomingMessage {
    /// Send by the client to authenticate it with the server. The server should not accept any
    /// messages before this message has been processed.
    Authenticate {
        client_info: ClientInfo,
        password: String,
    },

    /// Send by the client to indicate it wants to send a new file to the server.
    NotifyNewFile(PathBuf),
}

/// An outgoing message to the client (send by the server TO the client)
#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "payload", rename_all = "camelCase")]
pub enum OutgoingMessage {
    /// The remote client failed to send a message within the expected timeframe.
    Timeout,

    /// Send to the client if the server expected a message other than the one received.
    Conflict,

    /// Indicates that the web socket message was not in the correct format. Messages should use
    /// the [Message::Text] format to communicate.
    IncorrectFormat,

    /// Indicates that the client send a text message that contained incorrectly formatted JSON or
    /// could not be deserialized into an [IncomingMessage].
    FailedToParseJson(String),

    /// Indicates that the authentication attempt of the client failed. Server should close the
    /// connection after sending this message.
    AuthenticationFailed,

    /// Indicates to the client that it authenticated successfully and the server is now ready to
    /// start receiving messages from the client.
    AuthenticationSuccess,

    /// Indicates to the client that the server encountered an issue that is not the fault of the
    /// client.
    InternalServerError,
    Sync(RemoteOpLog),
}
