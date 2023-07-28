use async_trait::async_trait;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::connect_async;
use client::models::ws_messages::{WsClientIncoming, WsClientOutgoing};
use client::traits::ws_client::{WsClient, WsClientError};
use futures_util::stream::StreamExt;

pub struct TungsteniteWsClient {
    sender: Sender<WsClientOutgoing>,
    receiver: Receiver<WsClientIncoming>,
}

#[async_trait]
impl WsClient<WsClientOutgoing, WsClientIncoming> for TungsteniteWsClient {
    type Err = TungsteniteWsError;

    async fn connect(addr: String) -> Result<Self, Self::Err> {
        let incoming = mpsc::channel::<WsClientIncoming>(64);
        let outgoing = mpsc::channel::<WsClientOutgoing>(64);

        let (mut stream, _) = connect_async(addr)
            .await?;

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    v = stream.next() => {
                        dbg!(v);
                    }
                }
            }
        });

        Ok(Self {
            sender: outgoing.0,
            receiver: incoming.1,
        })
    }

    async fn send(&mut self, message: WsClientOutgoing) -> Result<(), Self::Err> {
        todo!()
    }

    async fn receive(&mut self) -> Result<WsClientIncoming, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Error)]
pub enum TungsteniteWsError {
    #[error("{0}")]
    TungsteniteError(#[from] tokio_tungstenite::tungstenite::Error),
}

impl WsClientError for TungsteniteWsError {}
