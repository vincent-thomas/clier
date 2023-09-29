mod commands_argv;
mod flags_argv;
mod utils;

use std::collections::{HashMap, HashSet};

use utils::strip_dash;

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
      .flat_map(|(index, flag)| -> Vec<(String, String)> {
        let is_long = is_long_flag(flag);
        if is_long {
          let key_and_value = strip_dash(flag);

          if key_and_value.contains('=') {
            let key_value_vec: Vec<&str> = key_and_value.split('=').collect();
            assert!(key_value_vec.len() == 2, "Invalid format");
            let key = key_value_vec.first().unwrap().to_string();
            let value = key_value_vec.last().unwrap().to_string();
            return vec![(key, value)];
          }

          if key_and_value.starts_with("no-") {
            let key = key_and_value.strip_prefix("no-").unwrap().to_string();
            return vec![(key, "false".to_string())];
          }

          let next_arg = args.get(index + 1);

          if next_arg.is_some_and(|value| is_short_flag(value) || is_long_flag(value)) {
            let key = key_and_value;
            return vec![(key, "true".to_string())];
          } else {
            let key = key_and_value;
            let value = "true".to_string();
            let value = next_arg.unwrap_or(&value);
            return vec![(key, value.to_string())];
          }
        };

        // Är kort flag
        let key_and_value = strip_dash(flag);
        let contains_equal = key_and_value.contains('=');
        let has_space =
          args.get(index + 1).is_some_and(|value| !is_short_flag(value) && !is_long_flag(value));

        if !contains_equal && !has_space {
          let test: Vec<(String, String)> =
            key_and_value.chars().map(|v| (v.to_string(), "true".to_string())).collect();
          return test;
        }

        let keys_value: Vec<&str> = key_and_value.split('=').collect();
        let short_keys = keys_value.first().unwrap().chars();

        let keys: Vec<(String, String)> = short_keys
          .clone()
          .map(|v| (v, "true".to_string()))
          .map(|value| {
            let mut keys_value = keys_value.clone();
            if keys_value.get(1).is_none() && keys_value.len() == 1 {
              let next_arg = args.get(index + 1);
              let valid =
                next_arg.is_some_and(|value| !(is_long_flag(value) || is_short_flag(value)));

              if valid {
                keys_value.push(next_arg.unwrap());
              }
            }
            let is_last_in_short_keys = short_keys.clone().last().unwrap() == value.0;
            if is_last_in_short_keys {
              if keys_value.get(1).is_some() {
                return (value.0.to_string(), keys_value.get(1).unwrap().to_string());
              } else {
                return (value.0.to_string(), "true".to_string());
              }
            }
            (value.0.to_string(), value.1.to_string())
          })
          .collect();

        keys
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
    "-vt".to_string(),
    "value".to_string(),
    "-fe=t".to_string(),
    "--no-fdsafsa".to_string(),
    "--test=value".to_string(),
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
