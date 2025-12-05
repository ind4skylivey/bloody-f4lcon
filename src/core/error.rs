use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FalconError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("config error: {0}")]
    Config(String),
    #[error("timeout while contacting provider: {0}")]
    Timeout(String),
    #[error("provider error: {0}")]
    Provider(String),
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("unknown error: {0}")]
    Unknown(String),
}

impl From<tokio::task::JoinError> for FalconError {
    fn from(err: tokio::task::JoinError) -> Self {
        FalconError::Unknown(err.to_string())
    }
}
