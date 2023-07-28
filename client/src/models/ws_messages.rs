use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub enum WsClientIncoming {}

#[derive(Debug, Serialize)]
pub enum WsClientOutgoing {}
