use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Client IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Bincode error: {0}")]
    Bincode(#[from] Box<bincode::ErrorKind>),
}
