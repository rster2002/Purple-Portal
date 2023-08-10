// pub mod traits;
// pub mod models;
// pub mod procedures;

use std::future::Future;
use std::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use crate::modules::ws_client::models::ws_messages::{IncomingMessage, OutgoingMessage};

pub struct ProcedureLoan(u32, Sender<IncomingMessage>);

type ProcedureExecutor = fn (incoming: Receiver<IncomingMessage>, outgoing: Sender<OutgoingMessage>) -> dyn Future<Output=()>;

pub struct Procedure {
    id: u32,
    executor: ProcedureExecutor,
    incoming_sender: Sender<IncomingMessage>,
}

impl Procedure {
    pub fn new(id: u32, executor: ProcedureExecutor) -> Self {
        let (incoming_sender, incoming_receiver) = tokio::sync::mpsc::channel::<IncomingMessage>(32);

        tokio::spawn(async move {

        });

        Self {
            id,
            executor,
            incoming_sender,
        }
    }

    pub fn create_loan(&self) -> ProcedureLoan {
        ProcedureLoan(
            self.id,
            self.incoming_sender.clone()
        )
    }
}
