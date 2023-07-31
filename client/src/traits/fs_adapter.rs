use std::error::Error;
use std::fmt::{Debug, Display};
use std::path::PathBuf;

use async_trait::async_trait;

pub trait FsAdapterError: Error {}

#[async_trait]
pub trait FsAdapter: Sync + Send {
    type Err: FsAdapterError;

    async fn read_file(&self, path: &PathBuf) -> Result<Vec<u8>, Self::Err>;
    async fn write_file(&self, path: &PathBuf, contents: &Vec<u8>) -> Result<(), Self::Err>;
    async fn read_dir(&self, path: &PathBuf) -> Result<Vec<PathBuf>, Self::Err>;

    async fn create_dir_all(&self, path: &PathBuf) -> Result<(), Self::Err>;

    async fn is_file(&self, path: &PathBuf) -> Result<bool, Self::Err>;
    async fn is_dir(&self, path: &PathBuf) -> Result<bool, Self::Err>;
    async fn exists(&self, path: &PathBuf) -> Result<bool, Self::Err>;
}
