use std::error::Error;
use std::fmt::{Display, Formatter};
use async_trait::async_trait;
use client::models::ws_messages::{WsClientIncoming, WsClientOutgoing};
use client::traits::ws_client::{WsClient, WsClientError};

pub struct JsWsClient;

#[derive(Debug)]
pub struct JsWsError;

impl Error for JsWsError {}

impl Display for JsWsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "JS ws error")
    }
}

impl WsClientError for JsWsError {}

#[async_trait]
impl WsClient<WsClientOutgoing, WsClientIncoming> for JsWsClient {
    type Err = JsWsError;

    async fn connect(addr: String) -> Result<Self, Self::Err> {
        todo!()
    }

    async fn send(&mut self, message: WsClientOutgoing) -> Result<(), Self::Err> {
        todo!()
    }

    async fn receive(&mut self) -> Result<WsClientIncoming, Self::Err> {
        todo!()
    }
}
