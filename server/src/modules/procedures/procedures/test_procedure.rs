use async_trait::async_trait;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::modules::procedures::traits::procedure::Procedure;

pub struct TestProcedure;

#[async_trait]
impl Procedure<String, String> for TestProcedure {
    async fn run(incoming: Receiver<String>, outgoing: Sender<String>) {
        todo!()
    }
}
