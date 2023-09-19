use crate::{command::Command, error::Error, prelude::CResult, Argv};
use std::collections::HashMap;

pub(crate) fn match_command(
  registered_commands: &[Command],
  commands: &[String],
) -> Option<Command> {
  let mut command_matcher = HashMap::new();
  for command in registered_commands.iter() {
    let mut command_name = command.name.to_string();
    command_matcher.insert(command_name.clone(), command.clone());
    let mut command_to_check = command.to_owned();

    loop {
      let t = command_to_check.clone();
      if t.children.is_some() && t.children.unwrap().is_empty() {
        panic!("Use None instead of empty vector");
      } else if command_to_check.clone().children.is_some() {
        for child in command_to_check.children.clone().unwrap() {
          command_name = format!("{}.{}", &command_name, child.name);
          command_matcher.insert(command_name.clone(), child.clone());
          command_to_check = child;
        }
      } else {
        break;
      }
    }
  }

  let to_run = command_matcher.get(&commands.join(".")).cloned();

  match to_run {
    Some(command) => Some(command),
    None => match commands.is_empty() {
      true => command_matcher.get("root").cloned(),
      false => None,
    },
  }
}

fn is_short_flag(flag: impl Into<String>) -> bool {
  let flag = flag.into();

  flag.starts_with('-') && flag[1..].len() == 1
}

fn is_long_flag(flag: impl Into<String>) -> bool {
  let flag = flag.into();
  flag.starts_with("--") && flag[2..].len() != 1
}

pub(crate) fn transform_vargs(args: &[String]) -> CResult<Argv> {
  let mut parsed_flags: HashMap<String, String> = HashMap::new();
  let mut error: Option<Error> = None;

  args
    .iter()
    .filter(|value| value.starts_with('-') || value.starts_with("--"))
    .enumerate()
    .filter_map(|(index, flag)| {
      let is_short = is_short_flag(flag);
      let is_long = is_long_flag(flag);
      let has_equal = flag.contains('=');

      let flag = if is_short {
        flag[1..].to_string()
      } else if is_long {
        flag[2..].to_string()
      } else {
        return None;
      };

      let (key, value) = if has_equal {
        let flag_vec = flag.split('=').collect::<Vec<&str>>();
        let key = flag_vec.first().unwrap().to_string();
        let value = flag_vec.get(1).unwrap().to_string();
        if key.starts_with("no-") {
          error =
            Some(Error::InvalidFormat("'no' modifier cannot be used with equal sign".to_string()));
        };

        (key, value)
      } else {
        let key = flag.clone();
        let default_flag = &"--tt".to_string();

        let next_arg = args.get(index + 1).unwrap_or(default_flag);
        let is_next_flag = is_short_flag(next_arg) || is_long_flag(next_arg);
        println!("NEXT_FLAG={:?} KEY={:?}", is_next_flag, key);
        let value = if is_next_flag {
          if key.starts_with("no-") {
            "false".to_string()
          } else {
            "true".to_string()
          }
        } else {
          if key.starts_with("no-") {
            "false".to_string()
          } else {
            next_arg.to_string()
          }
        };

        if let Some(key_no_no) = key.strip_prefix("no-") {
          (key_no_no.to_string(), value)
        } else {
          (key, value)
        }
      };

      Some(format!("{}={}", key, value))
    })
    .for_each(|value| {
      let flag = value.split('=').collect::<Vec<&str>>();
      let key = flag.first().unwrap();
      let value = flag.get(1).unwrap();
      parsed_flags.insert(key.to_string(), value.to_string());
    });
  let commands = args
    .iter()
    .enumerate()
    .filter(|(index, command_may)| {
      let index = (if *index == 0 { 1 } else { *index }) - 1;

      let is_current_flag = is_short_flag(*command_may) || is_long_flag(*command_may);
      if let Some(cmd_before) = args.get(index) {
        let is_before_flag = is_short_flag(cmd_before) || is_long_flag(cmd_before);
        if is_before_flag && !is_current_flag && cmd_before.starts_with("--no-") {
          return true;
        }
        if (is_before_flag) && !is_current_flag && !cmd_before.contains('=') {
          return false;
        }
      };
      !is_current_flag
    })
    .map(|v| v.1.clone())
    .collect::<Vec<String>>();

  match error {
    None => CResult::Ok(Argv { commands, flags: parsed_flags }),
    Some(err) => CResult::Err(err),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_transform_vargs() {
    let result = transform_vargs(&[
      "command".to_string(),
      "--test=value".to_string(),
      "--no-value".to_string(),
    ])
    .unwrap();
    let mut hash = HashMap::new();
    hash.insert("test".to_string(), "value".to_string());
    hash.insert("value".to_string(), "false".to_string());

    assert_eq!(*result.commands, vec!["command".to_string()]);
    assert_eq!(result.flags, hash);
  }
}
