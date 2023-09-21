use std::collections::{HashMap, HashSet};

use crate::error::Error;

pub(crate) type Flags = HashMap<String, String>;

#[derive(Debug, Clone, Default)]
pub struct Argv {
  pub commands: Vec<String>,
  pub flags: Flags,
}

fn is_short_flag(flag: impl Into<String>) -> bool {
  let flag = flag.into();

  flag.starts_with('-') && flag[1..].len() == 1
}

fn is_long_flag(flag: impl Into<String>) -> bool {
  let flag = flag.into();
  flag.starts_with("--") && flag[2..].len() != 1
}

fn strip_dash(is_long: bool, flag: impl Into<String>) -> Option<String> {
  let flag = flag.into();
  flag.strip_prefix(if is_long { "--" } else { "-" }).map(|v| v.to_string())
}

#[doc(hidden)]
pub fn transform(args: &[String]) -> Result<Argv, Error> {
  let mut parsed_flags: HashMap<String, String> = HashMap::new();
  let mut error: Option<Error> = None;

  let mut done_flags: HashSet<String> = HashSet::new();

  args
    .iter()
    .enumerate()
    .filter(|(_, value)| value.starts_with('-') || value.starts_with("--"))
    .map(|(index, flag)| {
      let has_equal = flag.contains('=');

      let flag_vec = flag.split('=').collect::<Vec<&str>>();
      let key = flag_vec.first().unwrap().to_string();

      let is_long = is_long_flag(key.clone());

      let key = strip_dash(is_long, key).unwrap();

      let (key, value) = if has_equal {
        let value = flag_vec.get(1).unwrap().to_string();
        if key.starts_with("no-") {
          error =
            Some(Error::InvalidFormat("'no' modifier cannot be used with equal sign".to_string()));
        };

        (key, value)
      } else {
        let case_to_fail = &"--tt".to_string();

        let next_arg = args.get(index + 1).unwrap_or(case_to_fail);
        let is_next_flag = is_short_flag(next_arg) || is_long_flag(next_arg);
        let value = if is_next_flag {
          if key.starts_with("no-") {
            "false".to_string()
          } else {
            "true".to_string()
          }
        } else if key.starts_with("no-") {
          "false".to_string()
        } else {
          next_arg.to_string()
        };

        match key.strip_prefix("no-") {
          Some(key_no_no) => (key_no_no.to_string(), value),
          None => (key.to_string(), value),
        }
      };
      if done_flags.contains(&key.to_string()) {
        error = Some(Error::ToManyFlags(key.to_string()));
      }

      done_flags.insert(key.to_string());

      (key, value)
    })
    .for_each(|(key, value)| {
      parsed_flags.insert(key, value);
    });
  let commands = args
    .iter()
    .enumerate()
    .filter(|(index, command_may)| {
      let index = (if *index == 0 { 1 } else { *index }) - 1;

      let is_current_flag = is_short_flag(*command_may) || is_long_flag(*command_may);
      match args.get(index) {
        None => !is_current_flag,
        Some(cmd_before) => {
          let is_before_flag = is_short_flag(cmd_before) || is_long_flag(cmd_before);
          if is_before_flag && !is_current_flag && cmd_before.starts_with("--no-") {
            return true;
          }
          if is_before_flag && !is_current_flag && !cmd_before.contains('=') {
            return false;
          }
          !is_current_flag
        }
      }
    })
    .filter(|(_, command)| !command.starts_with('-') || !command.starts_with("--"))
    .map(|v| v.1.clone())
    .collect::<Vec<String>>();

  match error {
    None => Result::Ok(Argv { commands, flags: parsed_flags }),
    Some(err) => Result::Err(err),
  }
}

#[test]
fn transform_vargs() {
  let result = transform(&[
    "command".to_string(),
    "--test=value".to_string(),
    "--name".to_string(),
    "test".to_string(),
    "--no-value".to_string(),
    "subcommand".to_string(),
  ])
  .unwrap();
  let mut hash = HashMap::new();
  hash.insert("name".to_string(), "test".to_string());
  hash.insert("value".to_string(), "false".to_string());
  hash.insert("test".to_string(), "value".to_string());

  assert_eq!(*result.commands, vec!["command".to_string(), "subcommand".to_string()]);
  assert_eq!(result.flags, hash);
}
