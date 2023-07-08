#![deny(unused_results)]
#![deny(clippy::unwrap_used)]

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::dir_hash_walker::DirHashWalker;
use crate::models::local_state::LocalState;
use crate::prelude::*;
use crate::state_manager::StateManager;
use crate::traits::fs_adapter::FsAdapter;

pub mod error;
pub(crate) mod prelude;
mod dir_hash_walker;
mod models;
pub mod state_manager;
pub(crate) mod utils;
pub mod traits;

pub struct PurplePortalClient<T>
    where T: FsAdapter,
{
    pub(crate) vault_root: PathBuf,
    pub(crate) config_root: PathBuf,
    pub(crate) fs_adapter: T,
}

impl<T> PurplePortalClient<T>
    where T: FsAdapter,
{
    pub async fn init(vault_root: PathBuf, fs_adapter: T) -> Result<Self> {
        let config_root = vault_root.join(".purple-portal");

        fs_adapter.create_dir_all(&config_root)
            .await?;

        Ok(Self {
            vault_root,
            config_root,
            fs_adapter,
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

        let local_state = state_manager.take_fs_snapshot()
            .await;

        Ok(())
    }

    pub async fn watch() -> () {
        todo!()
    }
}
