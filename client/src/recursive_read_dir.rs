use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use async_recursion::async_recursion;
use crate::prelude::*;
use crate::PurplePortalClient;
use crate::traits::fs_adapter::FsAdapter;

pub struct RecursiveReadDir<'a, T>
    where T: FsAdapter,
{
    client: &'a PurplePortalClient<T>,
}

impl<'a, T> RecursiveReadDir<'a, T>
    where T: FsAdapter,
{
    pub fn new(client: &'a PurplePortalClient<T>) -> Self {
        Self {
            client,
        }
    }

    /// Returns absolute paths
    pub async fn walk(&self, root: &PathBuf) -> Result<Vec<PathBuf>> {
        Ok(self.walk_dir(root)
            .await?)
    }

    #[async_recursion]
    async fn walk_dir(&self, path: &PathBuf) -> Result<Vec<PathBuf>> {
        let entries = self.client
            .fs_adapter
            .read_dir(path)
            .await?;

        let mut list = vec![];

        for entry in entries {
            if entry.file_name().unwrap().to_str().unwrap().starts_with(".") {
                continue;
            }

            if self.client.fs_adapter.is_dir(&entry).await? {
                let mut sub_dir = self.walk_dir(&entry)
                    .await?;

                list.append(&mut sub_dir);
            }

            if self.client.fs_adapter.is_file(&entry).await? {
                list.push(entry);
            }
        }

        Ok(list)
    }
}
