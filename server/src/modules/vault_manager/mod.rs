pub mod error;

use std::path::PathBuf;
use crate::modules::vault_manager::error::VaultError;
use crate::utils::safe_join::safe_join;

pub struct VaultManager {
    root: PathBuf,
}

impl VaultManager {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
        }
    }

    /// Securely resolves the path relative to the root.
    fn resolve_path(&self, path: PathBuf) -> Result<PathBuf, VaultError> {
        Ok(safe_join(&self.root, &path))
    }

    /// Checks whether the paths exists on the server. Returns true if any data is stored from any
    /// client.
    pub async fn exists(&self, path: PathBuf) -> Result<bool, VaultError> {
        todo!()
    }

    /// Checks whether any data exists for a specific id for the given path.
    pub async fn exists_for_id(&self, id: String, path: PathBuf) -> Result<bool, VaultError> {
        todo!()
    }
}
