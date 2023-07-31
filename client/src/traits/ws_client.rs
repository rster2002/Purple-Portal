use std::error::Error;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub trait WsClientError: Error {}

#[async_trait]
pub trait WsClient<O, I>: Sync + Send + Sized
where
    O: Serialize, // Outgoing
    I: for<'de> Deserialize<'de>,
{
    type Err: WsClientError;

    async fn connect(addr: String) -> Result<Self, Self::Err>;
    async fn send(&mut self, message: O) -> Result<(), Self::Err>;
    async fn receive(&mut self) -> Result<I, Self::Err>;
}
