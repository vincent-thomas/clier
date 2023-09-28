mod commands_argv;
mod flags_argv;
mod utils;

use std::collections::{HashMap, HashSet};

use self::utils::{is_long_flag, is_short_flag};

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
    let mut done_flags: HashSet<String> = HashSet::new();

    let flags_to_operate = args
      .iter()
      .enumerate()
      // Is valid flag?
      .filter(|(_, value)| is_long_flag(*value) || is_short_flag(*value))
      // Deeper validation
      .filter(|(_, flag)| {
        let has_equal = flag.contains('=');
        let flag_vec = flag.split('=').collect::<Vec<&str>>();
        let key = flag_vec.first().unwrap();
        if has_equal && key.starts_with("no-") || done_flags.contains(&key.to_string()) {
          false
        } else {
          done_flags.insert(key.to_string());
          true
        }
      })
      .map(|(index, flag)| {
        flags_argv::parse_flag(flag.clone(), args.get(index + 1).map(|v| v.as_str()))
      })
      .collect::<Vec<(String, String)>>();

    let mut flags: HashMap<String, String> = HashMap::with_capacity(flags_to_operate.len());

    for (key, value) in flags_to_operate {
      flags.insert(key, value);
    }

    let commands = args
      .iter()
      .enumerate()
      .filter(|(index, command_may)| commands_argv::filter_commands(index, command_may, args))
      .map(|v| v.1.clone())
      .collect::<Vec<String>>();

    Argv { commands, flags }
  }
}
#[test]
fn transform_vargs() {
  let shit: &[String] = &[
    "command".to_string(),
    "subcommand".to_string(),
    "--name=test".to_string(),
    "--value=false".to_string(),
    "-valu".to_string(),
    "--no-fdsafsa".to_string(),
    "--test=value".to_string(),
    "--test1".to_string(),
  ];

  let result = Argv::from(shit);
  let mut hash = HashMap::new();
  hash.insert("name".to_string(), "test".to_string());
  hash.insert("value".to_string(), "false".to_string());
  hash.insert("fdsafsa".to_string(), "false".to_string());
  hash.insert("test".to_string(), "value".to_string());
  hash.insert("test1".to_string(), "true".to_string());

  assert_eq!(*result.commands, vec!["command".to_string(), "subcommand".to_string()]);
  assert_eq!(result.flags, hash);
}
