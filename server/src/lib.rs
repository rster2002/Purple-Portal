#[macro_use]
extern crate rocket;

pub mod modules;
mod client;

use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use rand::{RngCore, thread_rng};
use rocket::futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Mutex, oneshot};
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use modules::cors;
use crate::client::Client;

pub struct PurplePortalServer {
    senders: Arc<Mutex<Vec<(u32, Sender<String>)>>>,
}

impl PurplePortalServer {
    pub fn new() -> Self {
        Self {
            senders: Arc::new(Mutex::new(vec![])),
        }
    }

    pub async fn start(&mut self) {
        let server = TcpListener::bind("127.0.0.1:9001")
            .await
            .unwrap();

        let (broadcast, mut rx) = mpsc::channel::<String>(100);

        let internal_arc = self.senders.clone();
        tokio::spawn(async move {
            loop {
                let Some(message) = rx.recv().await else {
                    break;
                };

                println!("Broadcast {}", &message);

                let guard = internal_arc.lock().await;

                for (id, sender) in guard.iter() {
                    println!("Message to {}", id);
                    sender.send(message.to_string())
                        .await
                        .unwrap();
                }
            }
        });

        let mut rnd = thread_rng();

        loop {
            let (stream, _) =  server.accept()
                .await
                .expect("Failed to start server");

            let id = rnd.next_u32();
            let sender = Client::start(
                id,
                stream,
                broadcast.clone(),
            )
                .await;

            self.senders.lock().await.push((id, sender));
            broadcast.clone().send("Hello world".to_string()).await.unwrap();
        }
    }
}
