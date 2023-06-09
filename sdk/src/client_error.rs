use crate::error::Error;
use thiserror::Error;
use tokio::io;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Invalid command")]
    InvalidCommand,
    #[error("Invalid transport {0}")]
    InvalidTransport(String),
    #[error("IO error")]
    IoError(#[from] io::Error),
    #[error("SDK error")]
    SdkError(#[from] Error),
}
