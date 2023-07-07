use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::dir_hash_walker::DirHashWalker;
use crate::models::local_state::LocalState;
use crate::PurplePortalClient;
use crate::prelude::*;
use crate::utils::diff::Diff;

pub struct StateManager<'a> {
    client: &'a PurplePortalClient,
}

impl<'a> StateManager<'a> {
    pub fn new(client: &'a PurplePortalClient) -> Self {
        Self {
            client,
        }
    }

    fn stored_state_path(&self) -> PathBuf {
        self.client.config_root.join(".local-state")
    }

    pub async fn get_fs_state(&self) -> Result<LocalState> {
        let hashes = DirHashWalker::walk(&self.client.vault_root)
            .await?;

        let files: HashMap<PathBuf, u64> = hashes
            .into_iter()
            .filter(|(path, _)| {
                !path.to_str()
                    .expect("Failed to convert to str")
                    .starts_with(".")
            })
            .collect();

        Ok(LocalState {
            hashes: files,
        })
    }

    pub async fn get_stored_state(&self) -> Result<LocalState> {
        let bin = tokio::fs::read(self.stored_state_path())
            .await?;

        Ok(bincode::deserialize(&bin)?)
    }

    pub async fn take_fs_snapshot(&self) -> Result<LocalState> {
        let local_state = self.get_fs_state()
            .await?;

        let i = bincode::serialize(&local_state)?;
        tokio::fs::write(self.stored_state_path(), i)
            .await?;

        Ok(local_state)
    }
}
