use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use async_trait::async_trait;
use crate::traits::fs_adapter::{FsAdapter, FsAdapterError};

pub struct TestFsAdapter;

#[derive(Debug)]
pub struct A;

impl Error for A {}

impl Display for A {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "A")
    }
}

impl FsAdapterError for A {}

#[async_trait]
impl FsAdapter for TestFsAdapter {
    type Err = A;

    async fn read_file(&self, path: &PathBuf) -> Result<Vec<u8>, Self::Err> {
        todo!()
    }

    async fn write_file(&self, path: &PathBuf, contents: &Vec<u8>) -> Result<(), Self::Err> {
        todo!()
    }

    async fn read_dir(&self, path: &PathBuf) -> Result<Vec<PathBuf>, Self::Err> {
        todo!()
    }

    async fn create_dir_all(&self, path: &PathBuf) -> Result<(), Self::Err> {
        todo!()
    }

    async fn is_file(&self, path: &PathBuf) -> Result<bool, Self::Err> {
        todo!()
    }

    async fn is_dir(&self, path: &PathBuf) -> Result<bool, Self::Err> {
        todo!()
    }

    async fn exists(&self, path: &PathBuf) -> Result<bool, Self::Err> {
        todo!()
    }
}
