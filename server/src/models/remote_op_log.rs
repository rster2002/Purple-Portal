use std::path::PathBuf;
use diamond_types::LocalVersion;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteOpLog {
    pub agent_id: String,
    pub path: PathBuf,
    pub last_sync: Option<LocalVersion>,
    pub op_log: Vec<u8>,
}
