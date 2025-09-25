use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("{0}")]
    OpenSsl(String),
}

