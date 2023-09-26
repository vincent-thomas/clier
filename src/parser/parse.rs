use super::commands_argv::filter_commands;
use super::flags_argv::parse_flag;
use std::collections::{HashMap, HashSet};

/// Example structure:
/// ```ignore
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
  /// Commands
  pub commands: Vec<String>,
  /// Flags
  pub flags: ParsedFlags,
}

pub(crate) type ParsedFlags = HashMap<String, String>;

impl From<&[String]> for Argv {
  fn from(args: &[String]) -> Self {
    let mut done_flags: HashSet<String> = HashSet::new();

    let flags_to_operate = args
      .iter()
      .enumerate()
      // Is flag
      .filter(|(_, value)| value.starts_with('-') || value.starts_with("--"))
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
      .collect::<Vec<(usize, &String)>>();

    let mut flags: HashMap<String, String> = HashMap::with_capacity(flags_to_operate.len());

    for (key, value) in flags_to_operate
      .into_iter()
      .map(|(index, flag)| parse_flag(flag.clone(), args.get(index + 1)))
    {
      flags.insert(key, value);
    }

    let commands = args
      .iter()
      .enumerate()
      .filter(|(index, command_may)| filter_commands(index, command_may, args))
      .filter(|(_, command)| !command.starts_with('-') || !command.starts_with("--"))
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
    "--test=value".to_string(),
  ];

  let result = Argv::from(shit);
  let mut hash = HashMap::new();
  hash.insert("name".to_string(), "test".to_string());
  hash.insert("value".to_string(), "false".to_string());
  hash.insert("test".to_string(), "value".to_string());

  assert_eq!(*result.commands, vec!["command".to_string(), "subcommand".to_string()]);
  assert_eq!(result.flags, hash);
}
