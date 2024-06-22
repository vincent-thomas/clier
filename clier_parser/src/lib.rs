#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

mod commands_argv;
mod flags;
mod utils;

use alloc::boxed::Box;
use alloc::vec::Vec;
use commands_argv::transform_command_argv;
use flags::transform_flags_argv;
use hashbrown::HashMap;
use utils::remove_dashdash;
/// Example structure:
/// ```markdown
/// Argv {
///   commands: Vec<Box<str>> [
///      "command",
///      "subcommand",
///    ],
///    flags: HashMap<Box<str>, Box<str>> {
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
  pub commands: Vec<Box<str>>,
  /// Flags from argv in a key-value format
  pub flags: HashMap<Box<str>, Box<str>>,
  after_double_dash: Box<str>,
}

impl Argv {
  pub fn parse(args: &[&str]) -> Self {
    Argv::from(args)
  }

  pub fn after_dashes(&self) -> &str {
    &self.after_double_dash
  }
  fn transform_vargs(args: &[&str]) -> Argv {
    let (args, after_double_dash) = remove_dashdash(args);
    let flags = transform_flags_argv(&args);
    let commands = transform_command_argv(&args);

    Argv { commands, flags, after_double_dash }
  }
}

impl From<&str> for Argv {
  fn from(args: &str) -> Self {
    let args = args.split(' ').collect::<Vec<&str>>();
    Argv::transform_vargs(&args)
  }
}
impl From<&[&str]> for Argv {
  fn from(args: &[&str]) -> Self {
    Argv::transform_vargs(args)
  }
}
