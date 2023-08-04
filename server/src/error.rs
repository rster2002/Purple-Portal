use std::io;
use thiserror::Error;
use crate::modules::ws_client::error::WsError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    WsError(#[from] WsError),

    #[error("{0}")]
    IoError(#[from] io::Error)
}
