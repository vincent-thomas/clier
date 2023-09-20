use thiserror::Error as ThisError;

#[derive(Debug, ThisError, Clone)]
pub enum Error {
  #[error("Invalid format: {0}")]
  InvalidFormat(String),
  #[error("Duplicate of flags: {0}")]
  ToManyFlags(String),
  #[error("Command not found: {0}")]
  CommandNotFound(String),
  #[error("Missing flag: {0}")]
  MissingFlag(String),
}
