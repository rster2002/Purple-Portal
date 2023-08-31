use rand::{RngCore, thread_rng};
use crate::modules::procedures::r#mod::procedure_executor::ProcedureExecutor;

pub struct ProcedureRef<I, O> {
    id: u32,
    executor: Box<dyn ProcedureExecutor<I, O>>,
}

impl<I, O> ProcedureRef<I, O> {
    pub fn new(executor: impl ProcedureExecutor<I, O>) -> Self {
        Self {
            id: thread_rng().next_u32(),
            executor: Box::new(executor),
        }
    }
}
