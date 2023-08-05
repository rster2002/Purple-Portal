use thiserror::Error;

#[derive(Debug, Error)]
pub enum VaultError {
    #[error("Vault error: {0}")]
    IO(#[from] std::io::Error),
}
