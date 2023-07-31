use std::error::Error;
use std::fmt::{Debug, Display};

use wasm_bindgen::prelude::*;

use client::traits::fs_adapter::{FsAdapter, FsAdapterError};
use client::PurplePortalClient;

use crate::js_fs_adapter::JsFsAdapter;
use crate::js_ws_client::JsWsClient;

mod js_fs_adapter;
mod js_ws_client;

#[wasm_bindgen]
pub async fn greet(name: &str) {
    let future = PurplePortalClient::init("./".into(), JsFsAdapter, JsWsClient).await;

    // future_to_promise(future)
}
