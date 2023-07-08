use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use wasm_bindgen::prelude::*;
use client::PurplePortalClient;

use wasm_bindgen_futures::future_to_promise;
use client::traits::fs_adapter::{FsAdapter, FsAdapterError};

#[wasm_bindgen(module = "/defined-in-js.js")]
extern "C" {
    fn foo();
}

#[wasm_bindgen]
pub async fn greet(name: &str) {
    let future = PurplePortalClient::init("./".into(), JsFsAdapter)
        .await;


    // future_to_promise(future)
}

struct JsFsAdapter;

#[derive(Debug)]
struct JsFsError;

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
}
