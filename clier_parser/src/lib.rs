#![doc = include_str!("../README.md")]
mod commands_argv;
mod flags;
mod transformer;
mod utils;

use std::{collections::HashMap, env::args};
use transformer::transform_vargs;

/// Example structure:
/// ```markdown
/// Argv {
///   commands: [
///      "command",
///      "subcommand",
///    ],
///    flags: {
///      "test": "value",
///      "production": "false",
///      "help": "true",
///      "try-me": "false",
///    }
/// }
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Argv {
  /// Commands from argv
  pub commands: Vec<String>,
  /// Flags from argv in a key-value format
  pub flags: HashMap<String, String>,
  after_double_dash: String,
}

impl From<&str> for Argv {
  fn from(args: &str) -> Self {
    transform_vargs(&args.split(' ').map(|s| s.to_string()).collect::<Vec<String>>())
  }
}
impl From<String> for Argv {
  fn from(args: String) -> Self {
    transform_vargs(&args.split(' ').map(|s| s.to_string()).collect::<Vec<String>>())
  }
}
impl From<&[String]> for Argv {
  fn from(args: &[String]) -> Self {
    transform_vargs(args)
  }
}

impl Argv {
  pub fn parse() -> Self {
    Argv::from(args().collect::<Vec<String>>().as_slice())
  }

  pub fn after_dashes(&self) -> &str {
    self.after_double_dash.as_str()
  }
}
