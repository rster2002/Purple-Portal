#[macro_use]
extern crate rocket;

pub mod modules;

use std::path::PathBuf;
use rocket::futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use modules::cors;

pub struct PurplePortalServer {
    username: String,
    password: String,
    port: u16,
    vault_root: PathBuf,
    senders: Vec<Sender<String>>,
}

impl PurplePortalServer {
    pub fn new(
        username: impl Into<String>,
        password: impl Into<String>,
        port: u16,
        vault_root: PathBuf,
    ) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            port,
            vault_root,
            senders: vec![],
        }
    }

    fn push_sender(&mut self, sender: Sender<String>) {
        self.senders.push(sender);
    }

    fn get_senders(&self) -> &Vec<Sender<String>> {
        &self.senders
    }

    pub async fn start(mut self) {
        let server = TcpListener::bind("127.0.0.1:9001")
            .await
            .unwrap();

        loop {
            let (steam, _) =  server.accept()
                .await
                .expect("Failed to start server");

            let (tx, mut rx) = mpsc::channel::<String>(100);

            self.push_sender(tx);

            tokio::spawn(async move {
                println!("Connected");

                let mut i = accept_async(steam)
                    .await
                    .expect("Failed to accept as websocket");

                loop {
                    tokio::select! {
                        v = i.next() => {
                            let Some(message_result) = v else {
                                println!("Disconnecting");
                                break;
                            };

                            let message: Message = message_result.unwrap();

                            match message {
                                Message::Text(string) => {
                                    for sender in self.get_senders() {
                                        sender.send(string.to_string())
                                            .await
                                            .expect("Failed");
                                    }
                                },
                                _ => println!("Unsupported message type"),
                            }
                        },

                        v = rx.recv() => {
                            let Some(message) = v else {
                                break;
                            };

                            i.send(Message::Text(message))
                                .await
                                .expect("Failed to send");
                        },
                    }
                }
            });
        }
    }
}
