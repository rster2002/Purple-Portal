#[macro_use]
extern crate rocket;

pub mod modules;
mod client;

use std::path::PathBuf;
use std::sync::Arc;
use rand::{RngCore, thread_rng};
use rocket::futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Mutex, oneshot, RwLock};
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use modules::cors;
use crate::client::Client;

pub struct PurplePortalServer {
    senders: Arc<RwLock<Vec<(u32, Sender<String>)>>>,
}

impl PurplePortalServer {
    pub fn new() -> Self {
        Self {
            senders: Arc::new(RwLock::new(vec![])),
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

                let guard = internal_arc.read().await;
                for (id, sender) in guard.iter() {
                    println!("Message to {}", id);
                    let result = sender.send(message.to_string())
                        .await;

                    if let Err(e) = result {
                        if !sender.is_closed() {
                            println!("{:?}", e);
                        }
                    }
                }
            }
        });

        let mut rnd = thread_rng();

        loop {
            let (stream, _) =  server.accept()
                .await
                .expect("Failed to start server");

            let id = rnd.next_u32();
            let client = Client::accept(
                id,
                stream,
                broadcast.clone()
            )
                .await
                .unwrap();

            let sender = client.get_new_sender();
            client.start();

            let local_sender = sender.clone();
            let local_senders_vec = self.senders.clone();
            tokio::spawn(async move {
                local_sender.closed().await;

                // Find the index for the sender. Scoped to make sure the read lock is used as short
                // as possible.
                let mut index_result = {
                    local_senders_vec.read()
                        .await
                        .iter()
                        .enumerate()
                        .find(|(_, (v, _))| v == &id)
                        .map(|i| i.0)
                };

                if let Some(index) = index_result {
                    println!("{} Removed sender", id);
                    local_senders_vec.write().await.swap_remove(index);
                }
            });

            self.senders.write().await.push((id, sender));
        }
    }
}
