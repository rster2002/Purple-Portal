use wasm_bindgen::prelude::*;
use client::PurplePortalClient;

use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
pub fn greet(name: &str) {
    let future = PurplePortalClient::init();
    future_to_promise(future)
}