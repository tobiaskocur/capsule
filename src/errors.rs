// all the errors that Capsule can throw
use thiserror::Error;
#[derive(Debug, Error)] // we create an Error type from thiserror so we dont have to implement Display or std::error::Error
pub enum CapsuleError {
    // wraps standard io errors so we can use ? on file operations and procfs reads
    #[error("io error: {0}")] // this implement the Display message
    Io(#[from] std::io::Error),

    // used for custom namespace-related failures
    #[error("namespace error: {0}")]
    Namespace(String),
}

// shortcut so we dont have to write std::result::Result<T, CapsuleError> everywhere
pub type Result<T> = std::result::Result<T, CapsuleError>;
