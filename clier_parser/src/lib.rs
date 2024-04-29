#![doc = include_str!("../README.md")]
mod commands_argv;
mod flags;
mod utils;

use commands_argv::transform_command_argv;
use flags::transform_flags_argv;
use std::collections::HashMap;
use std::env;
use utils::remove_dashdash;

/// Example structure:
/// ```markdown
/// Argv {
///   commands: Vec<String> [
///      "command",
///      "subcommand",
///    ],
///    flags: HashMap<String, String> {
///      "test": "value",
///      "production": "false",
///      "help": "true",
///      "try-me": "false",
///    }
///    after_double_dash: ""
/// }
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Argv {
  /// Commands from argv
  pub commands: Vec<String>,
  /// Flags from argv in a key-value format
  pub flags: HashMap<String, String>,
  after_double_dash: String
}

impl Argv {
  pub fn parse() -> Self {
    Argv::from(&env::args().collect::<Vec<String>>()[1..])
  }

  pub fn after_dashes(&self) -> &str {
    self.after_double_dash.as_str()
  }
  fn transform_vargs(args: &[String]) -> Argv {
    let (args, after_double_dash) = remove_dashdash(args);
    let flags = transform_flags_argv(&args);
    let commands = transform_command_argv(&args);

    Argv { commands, flags, after_double_dash }
  }
}

impl From<&str> for Argv {
  fn from(args: &str) -> Self {
    let args = args.split(' ').map(|value| value.to_string()).collect::<Vec<String>>();
    Argv::transform_vargs(&args)
  }
}
impl From<String> for Argv {
  fn from(args: String) -> Self {
    let args = args.split(' ').map(|value| value.to_string()).collect::<Vec<String>>();
    Argv::transform_vargs(&args)
  }
}
impl From<&[String]> for Argv {
  fn from(args: &[String]) -> Self {
    let args = args.iter().map(|value| value.to_string()).collect::<Vec<String>>();
    Argv::transform_vargs(&args)
  }
}
