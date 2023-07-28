use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use wasm_bindgen::prelude::*;
use client::traits::fs_adapter::{FsAdapter, FsAdapterError};

pub struct JsFsAdapter;

#[wasm_bindgen(module = "/defined-in-js.js")]
extern "C" {
    fn foo();
}

#[derive(Debug)]
pub struct JsFsError;

impl Error for JsFsError {}

impl Display for JsFsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "js error")
    }
}

impl FsAdapterError for JsFsError {}

#[async_trait::async_trait]
impl FsAdapter for JsFsAdapter {
    type Err = JsFsError;

    async fn read_file(&self, path: &PathBuf) -> Result<Vec<u8>, Self::Err> {
        foo();
        todo!()
    }

    async fn write_file(&self, path: &PathBuf, contents: &Vec<u8>) -> Result<(), Self::Err> {
        foo();
        todo!()
    }

    async fn read_dir(&self, path: &PathBuf) -> Result<Vec<PathBuf>, Self::Err> {
        foo();
        todo!()
    }

    async fn create_dir_all(&self, path: &PathBuf) -> Result<(), Self::Err> {
        foo();
        todo!()
    }

    async fn is_file(&self, path: &PathBuf) -> Result<bool, Self::Err> {
        foo();
        todo!()
    }

    async fn is_dir(&self, path: &PathBuf) -> Result<bool, Self::Err> {
        foo();
        todo!()
    }

    async fn exists(&self, path: &PathBuf) -> Result<bool, Self::Err> {
        foo();
        todo!()
    }
}
