use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::{accept_async, WebSocketStream};
use crate::modules::ws_client::error::WsError;
use crate::modules::ws_client::models::ws_messages::{IncomingMessage, OutgoingMessage};
use futures_util::stream::StreamExt;

pub mod models;
pub mod error;

pub struct WsClient {
    sender: Sender<OutgoingMessage>,
    receiver: Receiver<IncomingMessage>,
}

impl WsClient {
    pub async fn accept(stream: TcpStream) -> Result<Self, WsError> {
        let (incoming_sender, incoming_receiver) = tokio::sync::mpsc::channel(32);
        let (outgoing_sender, outgoing_receiver) = tokio::sync::mpsc::channel(32);

        let mut ws_stream = accept_async(stream)
            .await?;

        tokio::spawn(async move {
            loop {
                tokio::select! {
                    v = ws_stream.next() => {
                        let Some(message_result) = v else {
                            break;
                        };


                    },
                }
            }
        });

        Ok(Self {
            sender: outgoing_sender,
            receiver: incoming_receiver,
        })
    }

    pub async fn send(&self, message: OutgoingMessage) -> Result<(), WsError> {

        todo!()
    }

    pub async fn receive(&mut self) -> Result<IncomingMessage, WsError> {
        todo!()
    }
}
