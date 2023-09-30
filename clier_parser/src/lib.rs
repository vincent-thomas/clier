#![doc = include_str!("../README.md")]

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
  after_double_dash: String,
}

pub trait Parse {
  fn parse(args: &[String]) -> Self;
  fn after_dash(&self) -> &str;
}
impl From<&str> for Argv {
  fn from(args: &str) -> Self {
    transform_vargs(&args.split(' ').map(|s| s.to_string()).collect::<Vec<String>>())
  }
}

impl From<&[String]> for Argv {
  fn from(args: &[String]) -> Self {
    transform_vargs(args)
  }
}

impl Parse for Argv {
  fn parse(args: &[String]) -> Self {
    Argv::from(args)
  }

  fn after_dash(&self) -> &str {
    self.after_double_dash.as_str()
  }
}

// Testar redan commands. Behöver inte det nu
#[test]
fn test_transform_vargs() {
  let result = Argv::from(
    "command subcommand --name=test --value=false -fe=t -vt value -ui --test1 --no-fdsafsa test",
  );
  let mut hash = HashMap::new();
  hash.insert("name".to_string(), "test".to_string());
  hash.insert("value".to_string(), "false".to_string());
  hash.insert("fdsafsa".to_string(), "false".to_string());
  hash.insert("v".to_string(), "true".to_string());
  hash.insert("t".to_string(), "value".to_string());
  hash.insert("f".to_string(), "true".to_string());
  hash.insert("e".to_string(), "t".to_string());
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
}
