use thiserror::Error as ThisError;
/// Clier Errors for crate.
#[derive(Debug, ThisError, Clone)]
pub enum Error {
  /// Invalid format for {0}
  #[error("Invalid format: {0}")]
  InvalidFormat(String),
  /// To many flags
  #[error("Duplicate of flags: {0}")]
  ToManyFlags(String),
  /// Command is not found
  #[error("Command not found: {0}")]
  CommandNotFound(String),
  /// Missing flag
  #[error("Missing flag: {0}")]
  MissingFlag(String),
}
