use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;
use crate::modules::ws_client::models::ws_messages::IncomingMessage;
use crate::modules::ws_client::WsClient;

pub mod models;

pub struct SignallingManager {
    // incoming: Sender<IncomingMessage>,
    // clients: Arc<RwLock<Vec<WsClient<'a>>>>,
}

impl SignallingManager {
    pub fn new() -> Self {
        Self {}
        // // let (incoming_sender, mut incoming_receiver) = tokio::sync::mpsc::channel(32);
        // let base_clients = Arc::new(RwLock::new(vec![]));
        // //
        // // let clients = base_clients.clone();
        // // tokio::spawn(async move {
        // //     loop {
        // //         let Some(message) = incoming_receiver.recv().await else {
        // //             break;
        // //         };
        // //     }
        // // });
        //
        // Self {
        //     // incoming: incoming_sender,
        //     clients: base_clients,
        // }
    }

    pub async fn add_client<'a>(&mut self, client: WsClient) {
        // self.clients.write()
        //     .await
        //     .push(client);
    }
}
