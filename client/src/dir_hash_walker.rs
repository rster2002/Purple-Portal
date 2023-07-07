use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use async_recursion::async_recursion;
use crate::prelude::*;

pub struct DirHashWalker;

impl DirHashWalker {
    pub async fn walk(root: &PathBuf) -> Result<HashMap<PathBuf, u64>> {
        Ok(Self::walk_dir(root)
            .await?
            .into_iter()
            .map(|(path, hash)| {
                let relative_path = path.strip_prefix(root).expect("Failed to strip prefix").to_path_buf();

                (relative_path, hash)
            })
            .collect())
    }

    #[async_recursion]
    async fn walk_dir(path: &PathBuf) -> Result<HashMap<PathBuf, u64>> {
        let mut dir_entries = tokio::fs::read_dir(path)
            .await?;

        let mut map = HashMap::new();

        while let Some(entry) = dir_entries.next_entry().await? {
            let path = entry.path();

            if path.is_dir() {
                let sub_dir = DirHashWalker::walk_dir(&path)
                    .await?;

                for (path, hash) in sub_dir {
                    let _ = map.insert(path, hash);
                }
            }

            if path.is_file() {
                let mut contents = tokio::fs::read_to_string(&path)
                    .await?;

                let mut hasher = DefaultHasher::new();
                contents.hash(&mut hasher);
                let hash = hasher.finish();

                let _ = map.insert(path, hash);
            }
        }

        Ok(map)
    }
}
