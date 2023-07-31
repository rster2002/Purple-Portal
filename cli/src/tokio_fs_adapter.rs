use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use async_trait::async_trait;
use client::traits::fs_adapter::{FsAdapter, FsAdapterError};
use thiserror::Error;

pub struct TokioFsAdapter;

#[async_trait::async_trait]
impl FsAdapter for TokioFsAdapter {
    type Err = TokioFsError;

    async fn read_file(&self, path: &PathBuf) -> Result<Vec<u8>, Self::Err> {
        Ok(tokio::fs::read(path).await?)
    }

    async fn write_file(&self, path: &PathBuf, contents: &Vec<u8>) -> Result<(), Self::Err> {
        tokio::fs::write(path, contents).await?;
        Ok(())
    }

    async fn read_dir(&self, path: &PathBuf) -> Result<Vec<PathBuf>, Self::Err> {
        let mut dir = tokio::fs::read_dir(path).await?;

        let mut result = vec![];

        while let Some(entry) = dir.next_entry().await? {
            result.push(entry.path())
        }

        Ok(result)
    }

    async fn create_dir_all(&self, path: &PathBuf) -> Result<(), Self::Err> {
        tokio::fs::create_dir_all(path).await?;

        Ok(())
    }

    async fn is_file(&self, path: &PathBuf) -> Result<bool, Self::Err> {
        Ok(path.is_file())
    }

    async fn is_dir(&self, path: &PathBuf) -> Result<bool, Self::Err> {
        Ok(path.is_dir())
    }

    async fn exists(&self, path: &PathBuf) -> Result<bool, Self::Err> {
        Ok(path.exists())
    }
}

#[derive(Debug, Error)]
pub struct TokioFsError {
    #[from]
    inner: tokio::io::Error,
}

impl Display for TokioFsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.to_string())
    }
}

impl FsAdapterError for TokioFsError {}
