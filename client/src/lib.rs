#![deny(unused_results)]
#![deny(clippy::unwrap_used)]

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::recursive_read_dir::RecursiveReadDir;
use crate::models::local_state::LocalOpLog;
use crate::models::ws_messages::{WsClientIncoming, WsClientOutgoing};
use crate::prelude::*;
use crate::state_manager::StateManager;
use crate::traits::fs_adapter::FsAdapter;
use crate::traits::ws_client::WsClient;

pub mod error;
pub(crate) mod prelude;
mod recursive_read_dir;
pub mod models;
pub mod state_manager;
pub(crate) mod utils;
pub mod traits;

#[cfg(test)]
mod tests;

pub struct PurplePortalClient<T, C>
    where T: FsAdapter,
        C: WsClient<WsClientOutgoing, WsClientIncoming>,
{
    pub(crate) vault_root: PathBuf,
    pub(crate) config_root: PathBuf,
    pub(crate) fs_adapter: T,
    pub(crate) ws_client: C,
}

impl<T, C> PurplePortalClient<T, C>
    where T: FsAdapter,
        C: WsClient<WsClientOutgoing, WsClientIncoming>,
{
    pub async fn init(
        vault_root: PathBuf,
        fs_adapter: T,
        ws_client: C,
    ) -> Result<Self> {
        let config_root = vault_root.join(".purple-portal");

        fs_adapter.create_dir_all(&config_root)
            .await?;

        Ok(Self {
            vault_root,
            config_root,
            fs_adapter,
            ws_client,
        })
    }

    pub fn get_vault_name(&self) -> String {
        self.vault_root.file_name()
            .expect("File name should've be checked when creating the client")
            .to_str()
            .expect("Failed to convert to str")
            .to_string()
    }

    pub async fn run_sync(&self) -> Result<()> {
        let state_manager = StateManager::new(self);

        let logs = state_manager.diff_all()
            .await?;

        Ok(())
    }

    pub async fn watch() -> () {
        todo!()
    }
}
