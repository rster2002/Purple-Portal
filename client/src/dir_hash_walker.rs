use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use async_recursion::async_recursion;
use crate::prelude::*;
use crate::PurplePortalClient;
use crate::traits::fs_adapter::FsAdapter;

pub struct DirHashWalker<'a, T>
    where T: FsAdapter,
{
    client: &'a PurplePortalClient<T>
}

impl<'a, T> DirHashWalker<'a, T>
    where T: FsAdapter,
{
    pub fn new(client: &'a PurplePortalClient<T>) -> Self {
        Self {
            client,
        }
    }

    pub async fn walk(&self, root: &PathBuf) -> Result<HashMap<PathBuf, u64>> {
        Ok(self.walk_dir(root)
            .await?
            .into_iter()
            .map(|(path, hash)| {
                let relative_path = path.strip_prefix(root).expect("Failed to strip prefix").to_path_buf();

                (relative_path, hash)
            })
            .collect())
    }

    #[async_recursion]
    async fn walk_dir(&self, path: &PathBuf) -> Result<HashMap<PathBuf, u64>> {
        let entries = self.client
            .fs_adapter
            .read_dir(path)
            .await?;

        let mut map = HashMap::new();

        for entry in entries {
            if self.client.fs_adapter.is_dir(&entry).await? {
                let sub_dir = self.walk_dir(&entry)
                    .await?;

                for (path, hash) in sub_dir {
                    let _ = map.insert(path, hash);
                }
            }

            if self.client.fs_adapter.is_file(&entry).await? {
                let mut contents = self.client
                    .fs_adapter
                    .read_file(&entry)
                    .await?;

                let mut hasher = DefaultHasher::new();
                contents.hash(&mut hasher);
                let hash = hasher.finish();

                let _ = map.insert(entry, hash);
            }
        }

        Ok(map)
    }
}
