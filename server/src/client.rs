mod socket_message;

use rocket::form::error::Entity::Value;
use rocket::futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_tungstenite::{accept_async, WebSocketStream};
use tokio_tungstenite::tungstenite::{Message, WebSocket};
use crate::client::socket_message::ReceivedSocketMessage;

pub struct Client {
    id: u32,
    socket: WebSocketStream<TcpStream>,
    broadcast: Sender<String>,
    sender: Sender<String>,
    receiver: Receiver<String>,
}

impl Client {
    pub async fn accept(
        id: u32,
        stream: TcpStream,
        broadcast: Sender<String>,
    ) -> Result<Self, ()> {
        let (sender, receiver) = mpsc::channel::<String>(100);

        let socket = accept_async(stream)
            .await
            .unwrap();

        Ok(Self {
            id,
            socket,
            broadcast,
            sender,
            receiver,
        })
    }

    pub fn get_new_sender(&self) -> Sender<String> {
        self.sender.clone()
    }

    pub fn start(mut self) -> () {
        tokio::spawn(async move {
            println!("{} Connected", self.id);

            let Some(authentication_message) = Self::next_message(&mut self.socket).await else {
                return;
            };

            dbg!(authentication_message);

            loop {
                println!("{} Loop", self.id);

                tokio::select! {
                    v = Self::next_message(&mut self.socket) => {
                        dbg!(v);
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
        });

        // sender
    }

    async fn next_message(socket: &mut WebSocketStream<TcpStream>) -> Option<ReceivedSocketMessage> {
        let Some(Ok(Message::Text(string))) = socket.next().await else {
            return None;
        };

        let Ok(message) = serde_json::from_str::<ReceivedSocketMessage>(&*string) else {
            return None;
        };

        Some(message)
    }
}
