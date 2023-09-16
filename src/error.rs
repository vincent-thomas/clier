use thiserror::Error as ThisError;

#[derive(Debug, ThisError, Clone)]
pub enum Error {
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    #[error("The meta function is required for clier in runnable mode.")]
    NoMeta,
    #[error("Command not found: {0}")]
    CommandNotFound(String),
}
