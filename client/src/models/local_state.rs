use std::collections::HashMap;
use std::path::PathBuf;
use diamond_types::LocalVersion;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalOpLog {
    pub last_sync: Option<LocalVersion>,
    pub op_log: Vec<u8>,
}
