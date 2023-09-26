mod use_flag;
mod use_flags;

pub use use_flag::*;
pub use use_flags::*;

/// FlagError
#[derive(Debug, Clone)]
pub enum FlagError {
  /// .
  InvalidFormat,
  /// .
  Unexisting,
  /// .
  ParseIntError,
}
