use std::collections::HashMap;
use std::path::{Path, PathBuf};
use similar::{Change, ChangeTag, TextDiff};

use crate::recursive_read_dir::RecursiveReadDir;
use crate::models::local_state::LocalState;
use crate::prelude::*;
use crate::PurplePortalClient;
use crate::traits::fs_adapter::FsAdapter;
use crate::utils::diff::Diff;

pub struct StateManager<'a, T>
    where T: FsAdapter,
{
    client: &'a PurplePortalClient<T>,
}

impl<'a, T> StateManager<'a, T>
    where T: FsAdapter
{
    pub fn new(client: &'a PurplePortalClient<T>) -> Self {
        Self {
            client,
        }
    }

    fn diff_root_path(&self) -> PathBuf {
        self.client
            .config_root
            .join("diff")
    }

    pub async fn diff_all(&self) -> Result<()> {
        let reader = RecursiveReadDir::new(self.client);
        let paths = reader.walk(&self.client.vault_root)
            .await?
            .into_iter()
            .filter(|p| {
                p.to_str().unwrap().ends_with(".md")
            });

        for path in paths {
            let i = self.diff_path(&path)
                .await;
        }

        Ok(())
    }

    pub async fn diff_path(&self, abs_path: &PathBuf) -> Result<()> {
        let relative_path = abs_path.strip_prefix(&self.client.vault_root)
            .unwrap()
            .to_path_buf();

        let diff_path = self.diff_root_path()
            .join(relative_path);

        let current_content = String::from_utf8(self.client
            .fs_adapter
            .read_file(&abs_path)
            .await?)
            .expect("Remove");

        if !self.client.fs_adapter.exists(&diff_path).await? {
            dbg!("New");
            let diff: Vec<Change<&str>> = TextDiff::from_chars("", &current_content)
                .iter_all_changes()
                .collect();

            dbg!(&diff);
        } else {
            // dbg!("Old");
            let diff_content = String::from_utf8(self.client
                .fs_adapter
                .read_file(&diff_path)
                .await?)
                .expect("Remove");

            let diff: Vec<Change<&str>> = TextDiff::from_chars(&diff_content, &current_content)
                .iter_all_changes()
                .filter(|x| {
                    !matches!(x.tag(), ChangeTag::Equal)
                })
                .collect();

            dbg!(&diff);
        }

        self.client
            .fs_adapter
            .create_dir_all(&diff_path.parent().unwrap().to_path_buf())
            .await?;

        // TODO this should be updated by the server
        // self.client
        //     .fs_adapter
        //     .write_file(&diff_path, &current_content.into_bytes())
        //     .await?;

        Ok(())
    }

    // pub async fn diff_path(&self, path: &PathBuf) {
    //
    // }

    // fn stored_state_path(&self) -> PathBuf {
    //     self.client.config_root.join(".local-state")
    // }
    //
    // pub async fn get_fs_state(&self) -> Result<LocalState> {
    //     let dir_hash_walker = DirHashWalker::new(self.client);
    //
    //     let hashes = dir_hash_walker.walk(&self.client.vault_root)
    //         .await?;
    //
    //     let files: HashMap<PathBuf, u64> = hashes
    //         .into_iter()
    //         .filter(|(path, _)| {
    //             !path.to_str()
    //                 .expect("Failed to convert to str")
    //                 .starts_with(".")
    //         })
    //         .collect();
    //
    //     Ok(LocalState {
    //         hashes: files,
    //     })
    // }
    //
    // pub async fn get_stored_state(&self) -> Result<LocalState> {
    //     let binary_content = self.client
    //         .fs_adapter
    //         .read_file(&self.stored_state_path())
    //         .await?;
    //
    //     Ok(bincode::deserialize(&binary_content)?)
    // }
    //
    // pub async fn take_fs_snapshot(&self) -> Result<LocalState> {
    //     let local_state = self.get_fs_state()
    //         .await?;
    //
    //     let binary_content = bincode::serialize(&local_state)?;
    //     self.client
    //         .fs_adapter
    //         .write_file(&self.stored_state_path(), &binary_content)
    //         .await?;
    //
    //     Ok(local_state)
    // }
}
