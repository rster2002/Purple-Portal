mod client_error;
mod socket_message;
mod handle_incoming_message;

use crate::client::client_error::ClientError;
use crate::client::socket_message::{ErrorMessage, IncomingSocketMessage, OutgoingSocketMessage};
use rocket::futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{accept_async, WebSocketStream};
use crate::models::client_info::ClientInfo;

pub struct Client {
    id: u32,
    socket: WebSocketStream<TcpStream>,
    broadcast: Sender<IncomingSocketMessage>,
    sender: Sender<OutgoingSocketMessage>,
    receiver: Receiver<OutgoingSocketMessage>,
}

impl Client {
    pub async fn accept(id: u32, stream: TcpStream, broadcast: Sender<IncomingSocketMessage>) -> Result<Self, ()> {
        let (sender, receiver) = mpsc::channel::<OutgoingSocketMessage>(100);

        let socket = accept_async(stream).await.unwrap();

        Ok(Self {
            id,
            socket,
            broadcast,
            sender,
            receiver,
        })
    }

    pub fn get_new_sender(&self) -> Sender<OutgoingSocketMessage> {
        self.sender.clone()
    }

    pub fn start(mut self) -> () {
        tokio::spawn(async move {
            let result: Result<(), ClientError> = async {
                println!("{} Connected", self.id);

                let client_info = self.await_authentication()
                    .await?;

                self.send_message(OutgoingSocketMessage::AuthenticationSuccess)
                    .await?;

                loop {
                    println!("{} Loop", self.id);

                    tokio::select! {
                        v = Self::next_message(&mut self.socket) => {
                            let Some(message) = v else {
                                return Ok(());
                            };

                            self.handle_incoming_message(&client_info, message).await?;
                        },

                        v = self.receiver.recv() => {
                            println!("{} Sending message", self.id);
                            let Some(message) = v else {
                                break;
                            };

                            self.socket.send(Message::Text(message)).await;
                        },
                    }
                }

                Ok(())
            }
            .await;

            if let Err(e) = result {
                let _ = self
                    .send_message(OutgoingSocketMessage::ClientError(ErrorMessage {
                        error: e.type_string(),
                        message: e.to_string(),
                    }))
                    .await;
            }
        });
    }

    async fn await_authentication(&mut self) -> Result<ClientInfo, ClientError> {
        let Some(authentication_message) = Self::next_message(&mut self.socket).await else {
            return Err(ClientError::AuthenticationFailed);
        };

        let IncomingSocketMessage::Authenticate { client_info, password } = authentication_message else {
            return Err(ClientError::AuthenticationFailed);
        };

        if password != "abc" {
            return Err(ClientError::AuthenticationFailed);
        }

        Ok(client_info)
    }

    async fn next_message(
        socket: &mut WebSocketStream<TcpStream>,
    ) -> Option<IncomingSocketMessage> {
        let option = socket.next().await;

        let Some(Ok(Message::Text(string))) = option else {
            return None;
        };

        let Ok(message) = serde_json::from_str::<IncomingSocketMessage>(&*string) else {
            return None;
        };

        Some(message)
    }

    async fn send_message(&mut self, message: OutgoingSocketMessage) -> Result<(), ClientError> {
        let message_json = serde_json::to_string(&message)?;

        self.socket.send(Message::Text(message_json)).await?;

        Ok(())
    }

    async fn handle_incoming_message(
        &mut self,
        client_info: &ClientInfo,
        message: IncomingSocketMessage,
    ) -> Result<(), ClientError> {
        match &message {
            IncomingSocketMessage::UnprocessableContent => {}
            IncomingSocketMessage::Authenticate { .. } => {
                self.send_message(OutgoingSocketMessage::IncorrectTime)
                    .await?;
            }
            _ => {
                self.broadcast.send(message)
                    .await?;
            }
        }

        Ok(())
    }
}
