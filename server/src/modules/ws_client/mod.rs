use std::time::Duration;
use futures_util::SinkExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::{accept_async, WebSocketStream};
use crate::modules::ws_client::error::WsError;
use crate::modules::ws_client::models::ws_messages::{IncomingMessage, OutgoingMessage};
use futures_util::stream::StreamExt;
use tokio_tungstenite::tungstenite::Message;

pub mod models;
pub mod error;

const WS_TIMEOUT: u64 = 1000;

pub struct WsClient {
    ws_stream: WebSocketStream<TcpStream>,
    // sender: Sender<OutgoingMessage>,
    // receiver: Receiver<IncomingMessage>,
}

impl WsClient {
    pub async fn accept(stream: TcpStream) -> Result<(Sender<OutgoingMessage>, Receiver<IncomingMessage>), WsError> {
        let (incoming_sender, incoming_receiver) = tokio::sync::mpsc::channel(32);
        let (outgoing_sender, outgoing_receiver) = tokio::sync::mpsc::channel(32);

        let ws_stream = accept_async(stream)
            .await?;

        let mut client = WsClient { ws_stream };

        tokio::spawn(async move {
            let result = client.run_loop()
                .await;

            if let Err(error) = result {
                if let WsError::OutgoingError(outgoing) = error {
                    let _ = client.send(outgoing)
                        .await;
                } else {
                    let _ = client.send(OutgoingMessage::InternalServerError)
                        .await;
                }
            }
        });

        Ok((outgoing_sender, incoming_receiver))
    }

    async fn run_loop(&mut self) -> Result<(), WsError> {
        let authentication_message = self.receive_with_timeout()
            .await?;

        let IncomingMessage::Authenticate { password, client_info } = authentication_message else {
            return Err(OutgoingMessage::Conflict.into());
        };

        if password != "abc".to_string() {
            return Err(OutgoingMessage::AuthenticationFailed.into());
        }

        self.send(OutgoingMessage::AuthenticationSuccess)
            .await?;

        loop {
            tokio::select! {
                v = self.receive() => {
                    if let Err(ws_error) = v {
                        if let WsError::OutgoingError(outgoing) = ws_error {
                            self.send(outgoing)
                                .await?;

                            continue;
                        } else {
                            return Err(ws_error);
                        }
                    }
                },
            }
        }
    }

    pub async fn send(&mut self, message: OutgoingMessage) -> Result<(), WsError> {
        let json_string = serde_json::to_string(&message)?;

        self.ws_stream.send(Message::Text(json_string))
            .await?;

        Ok(())
    }

    pub async fn receive(&mut self) -> Result<IncomingMessage, WsError> {
        let message = self.ws_stream.next()
            .await
            .ok_or(WsError::SocketClosed)??;

        Self::handle_message(message)
    }

    pub async fn receive_with_timeout(&mut self) -> Result<IncomingMessage, WsError> {
        tokio::select! {
            v = self.receive() => {
                v
            },

            v = tokio::time::sleep(Duration::from_millis(WS_TIMEOUT)) => {
                Err(OutgoingMessage::Timeout.into())
            },
        }
    }

    fn handle_message(message: Message) -> Result<IncomingMessage, WsError> {
        let Message::Text(string) = message else {
            return Err(OutgoingMessage::IncorrectFormat.into());
        };

        let incoming_message = match serde_json::from_str(&string) {
            Ok(message) => message,
            Err(error) => {
                return Err(OutgoingMessage::FailedToParseJson(error.to_string()).into());
            },
        };

        Ok(incoming_message)
    }
}
