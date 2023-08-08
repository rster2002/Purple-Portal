use std::sync::Arc;
use tokio::net::TcpListener;

use tokio::sync::RwLock;
use crate::modules::procedures::models::procedure_kind::ProcedureKind;
use crate::modules::procedures::traits::procedure::Procedure;
use crate::modules::signalling_manager::SignallingManager;
use crate::modules::ws_client::WsClient;

mod error;
mod modules;
mod utils;

pub struct PurplePortalServer {
    signalling_manager: SignallingManager,
}

impl PurplePortalServer {
    pub fn new() -> Self {
        Self {
            signalling_manager: SignallingManager::new(),
        }
    }

    pub async fn start(&mut self) -> Result<(), error::Error> {
        let server = TcpListener::bind("127.0.0.1:9001")
            .await?;

        println!("Listening to socket");
        loop {
            let accept_result = server.accept().await;

            let Ok((stream, _)) = accept_result else {
                dbg!(accept_result);
                continue;
            };

            let client = WsClient::accept(stream)
                .await?;

            self.signalling_manager.add_client(client)
                .await;
        }

        // let server = TcpListener::bind("127.0.0.1:9001").await.unwrap();
        //
        // let (broadcast, mut rx) = mpsc::channel::<String>(100);
        //
        // let internal_arc = self.senders.clone();
        // tokio::spawn(async move {
        //     loop {
        //         let Some(message) = rx.recv().await else {
        //             break;
        //         };
        //
        //         println!("Broadcast {}", &message);
        //
        //         let guard = internal_arc.read().await;
        //         for (id, sender) in guard.iter() {
        //             println!("Message to {}", id);
        //             let result = sender.send(message.to_string()).await;
        //
        //             if let Err(e) = result {
        //                 if !sender.is_closed() {
        //                     println!("{:?}", e);
        //                 }
        //             }
        //         }
        //     }
        // });
        //
        // let mut rnd = thread_rng();
        //
        // loop {
        //     let (stream, _) = server.accept().await.expect("Failed to start server");
        //
        //     dbg!("Incoming connection");
        //
        //     let id = rnd.next_u32();
        //     let client = Client::accept(id, stream, broadcast.clone()).await.unwrap();
        //
        //     let sender = client.get_new_sender();
        //     client.start();
        //
        //     let local_sender = sender.clone();
        //     let local_senders_vec = self.senders.clone();
        //     tokio::spawn(async move {
        //         local_sender.closed().await;
        //
        //         // Find the index for the sender. Scoped to make sure the read lock is used as short
        //         // as possible.
        //         let mut index_result = {
        //             local_senders_vec
        //                 .read()
        //                 .await
        //                 .iter()
        //                 .enumerate()
        //                 .find(|(_, (v, _))| v == &id)
        //                 .map(|i| i.0)
        //         };
        //
        //         if let Some(index) = index_result {
        //             println!("{} Removed sender", id);
        //             local_senders_vec.write().await.swap_remove(index);
        //         }
        //     });
        //
        //     self.senders.write().await.push((id, sender));
        //     dbg!("Connected");
        // }
    }
}
