//! # Command Line Argument Parser for Rust
//! `clier_parser` is a command line argument parser for rust.
//!
//! ## Parser
//! To start a new cli projects run:
//!
//! ```console
//! $ cargo new demo && cd demo
//! $ cargo add clier_parser
//! ```
//!
//! Then define your CLI in `src/main.rs`:
//!
//! ```rust
#![doc = include_str!("../examples/simple.rs")]
//! ```
//!
//! And try it out:
//! ```md
//! $ cargo run -- command subcommand -tfv testing --test=value --no-production --help
//! Argv {
//!     commands: [
//!         "command",
//!         "subcommand",
//!     ],
//!     flags: {
//!         "test": "value",
//!         "production": "false",
//!         "help": "true",
//!         "try-me": "false",
//!         "t": "true",
//!         "f": "true",
//!         "v": "testing"
//!     }
//! }
//! ```
//!

mod commands_argv;
mod flags_argv;
mod transformer;
mod utils;

use std::collections::HashMap;

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
}

impl From<&[String]> for Argv {
  fn from(args: &[String]) -> Self {
    transform_vargs(args)
  }
}

#[test]
fn test_transform_vargs() {
  let shit: &[String] = &[
    "command".to_string(),
    "subcommand".to_string(),
    "--name=test".to_string(),
    "--value=false".to_string(),
    "-vt".to_string(),
    "value".to_string(),
    "-fe=t".to_string(),
    "--no-fdsafsa".to_string(),
    "--test=value".to_string(),
    "-ui".to_string(),
    "--test1".to_string(),
  ];

  let result = Argv::from(shit);
  let mut hash = HashMap::new();
  hash.insert("name".to_string(), "test".to_string());
  hash.insert("value".to_string(), "false".to_string());
  hash.insert("fdsafsa".to_string(), "false".to_string());
  hash.insert("v".to_string(), "true".to_string());
  hash.insert("t".to_string(), "value".to_string());
  hash.insert("f".to_string(), "true".to_string());

  hash.insert("e".to_string(), "t".to_string());

  hash.insert("test".to_string(), "value".to_string());
  hash.insert("u".to_string(), "true".to_string());
  hash.insert("i".to_string(), "true".to_string());
  hash.insert("test1".to_string(), "true".to_string());

  for flag in hash.clone() {
    let left = result.flags.get(&flag.0);
    let right = Some(&flag.1);
    if left != right {
      panic!("Not right flag: {}: {}={:?}", flag.0, flag.1, left);
    }
  }

  assert_eq!(*result.commands, vec!["command".to_string(), "subcommand".to_string()]);
}
