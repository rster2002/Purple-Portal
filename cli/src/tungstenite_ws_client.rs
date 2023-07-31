use async_trait::async_trait;
use client::models::ws_messages::{WsClientIncoming, WsClientOutgoing};
use client::traits::ws_client::{WsClient, WsClientError};
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::{connect, Message};

pub struct TungsteniteWsClient {
    outgoing: Sender<WsClientOutgoing>,
    incoming: Receiver<WsClientIncoming>,
}

#[derive(Debug, Error)]
pub enum TungsteniteWsError {
    #[error("{0}")]
    TungsteniteError(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("{0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("The channel for incoming messages has ended")]
    IncomingChannelEnded,

    #[error("InternalIncomingChannelSendError: {0}")]
    InternalIncomingChannelSendError(#[from] mpsc::error::SendError<WsClientIncoming>),

    #[error("InternalOutgoingChannelSendError: {0}")]
    InternalOutgoingChannelSendError(#[from] mpsc::error::SendError<WsClientOutgoing>),
}

impl WsClientError for TungsteniteWsError {}

#[async_trait]
impl WsClient<WsClientOutgoing, WsClientIncoming> for TungsteniteWsClient {
    type Err = TungsteniteWsError;

    async fn connect(addr: String) -> Result<Self, Self::Err> {
        let (incoming_sender, incoming_receiver) = mpsc::channel::<WsClientIncoming>(64);
        let (outgoing_sender, mut outgoing_receiver) = mpsc::channel::<WsClientOutgoing>(64);

        let (mut stream, _) = connect_async(addr).await?;

        let local_outgoing_sender = outgoing_sender.clone();
        tokio::spawn(async move {
            let i: Result<(), TungsteniteWsError> = loop {
                let result: Result<(), TungsteniteWsError> = tokio::select! {
                    v = stream.next() => {
                        let Some(message) = v else {
                            return Ok::<(), TungsteniteWsError>(());
                        };

                        let Message::Text(string_content) = message? else {
                            local_outgoing_sender.send(WsClientOutgoing::UnprocessableContent)
                                .await?;

                            continue;
                        };

                        let Ok(incoming_message) = serde_json::from_str(&string_content) else {
                            local_outgoing_sender.send(WsClientOutgoing::UnprocessableContent)
                                .await?;

                            continue;
                        };

                        incoming_sender.send(incoming_message).await?;

                        Ok(())
                    },

                    v = outgoing_receiver.recv() => {
                        let Some(outgoing_message) = v else {
                            return Ok::<(), TungsteniteWsError>(());
                        };

                        stream.send(Message::Text(serde_json::to_string(&outgoing_message)?))
                            .await?;

                        Ok(())
                    },
                };

                result?;
            };

            if let Err(e) = i {
                dbg!(e);
            };
        });

        Ok(Self {
            outgoing: outgoing_sender,
            incoming: incoming_receiver,
        })
    }

    async fn send(&mut self, message: WsClientOutgoing) -> Result<(), Self::Err> {
        self.outgoing.send(message).await.unwrap();
        Ok(())
    }

    async fn receive(&mut self) -> Result<WsClientIncoming, Self::Err> {
        self.incoming
            .recv()
            .await
            .ok_or(TungsteniteWsError::IncomingChannelEnded)
    }
}
