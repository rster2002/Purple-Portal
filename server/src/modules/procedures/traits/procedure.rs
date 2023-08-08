use std::future::Future;
use async_trait::async_trait;
use tokio::sync::mpsc::{Receiver, Sender};
use serde::{Deserialize, Serialize};
use crate::modules::procedures::models::procedure_kind::ProcedureKind;

#[async_trait]
pub trait Procedure<I, O>
    where I: for<'de> Deserialize<'de>,
          O: Serialize
{
    async fn run(incoming: Receiver<I>, outgoing: Sender<O>);
}
