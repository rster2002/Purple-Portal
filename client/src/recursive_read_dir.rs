use crate::models::ws_messages::{WsClientIncoming, WsClientOutgoing};
use crate::prelude::*;
use crate::traits::fs_adapter::FsAdapter;
use crate::traits::ws_client::WsClient;
use crate::PurplePortalClient;
use async_recursion::async_recursion;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

pub struct RecursiveReadDir<'a, T, C>
where
    T: FsAdapter,
    C: WsClient<WsClientOutgoing, WsClientIncoming>,
{
    client: &'a PurplePortalClient<T, C>,
}

impl<'a, T, C> RecursiveReadDir<'a, T, C>
where
    T: FsAdapter,
    C: WsClient<WsClientOutgoing, WsClientIncoming>,
{
    pub fn new(client: &'a PurplePortalClient<T, C>) -> Self {
        Self { client }
    }

    /// Returns absolute paths
    pub async fn walk(&self, root: &PathBuf) -> Result<Vec<PathBuf>> {
        Ok(self.walk_dir(root).await?)
    }

    #[async_recursion]
    async fn walk_dir(&self, path: &PathBuf) -> Result<Vec<PathBuf>> {
        let entries = self.client.fs_adapter.read_dir(path).await?;

        let mut list = vec![];

        for entry in entries {
            if entry
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with(".")
            {
                continue;
            }

            if self.client.fs_adapter.is_dir(&entry).await? {
                let mut sub_dir = self.walk_dir(&entry).await?;

                list.append(&mut sub_dir);
            }

            if self.client.fs_adapter.is_file(&entry).await? {
                list.push(entry);
            }
        }

        Ok(list)
    }
}
