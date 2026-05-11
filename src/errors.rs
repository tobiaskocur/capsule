// all the errors that Capsule can throw
use thiserror::Error;
#[derive(Debug, Error)]
pub enum CapsuleError {

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("namespace error: {0}")]
    Namespace(String),
}

pub type Result<T> = std::result::Result<T, CapsuleError>;