use crate::{command::Command, Argv};
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

fn get_value_flag(flag: String) -> (String, String) {
  let is_splittable: bool = flag.contains('=');

  if !is_splittable {
    let is_false = flag.starts_with("no-");
    let key = flag.replace("no-", "");

    return match is_false {
      true => (key, "false".to_string()),
      false => (key, "true".to_string()),
    };
  }

  let flag = flag.split('=').collect::<Vec<&str>>();

  let flag_key = flag.first().unwrap().to_string();
  let flag_value = flag.get(1).unwrap().to_string();

  (flag_key, flag_value)
}

fn parse_flags(raw_flags: &[String]) -> Vec<(String, String)> {
  raw_flags
    .iter()
    .filter(|flag| {
      let flag: Vec<&str> = flag.split('=').collect();
      let key = flag.first().unwrap();

      key.starts_with('-') && key[1..].len() == 1 || key.starts_with("--") && key[2..].len() != 1
    })
    .map(|v| -> String {
      let flag: Vec<&str> = v.split('=').collect();
      let key = *flag.first().unwrap();
      let is_short = key.starts_with('-') && key[1..].len() == 1;
      if is_short {
        v[1..].to_string()
      } else {
        v[2..].to_string()
      }
    })
    .map(get_value_flag)
    .collect()
}

pub(crate) fn transform_vargs(args: &[String]) -> Argv {
  let mut commands_to_parse = vec![];
  let mut only_flags_raw: Vec<String> = vec![];

  args.iter().for_each(|value| {
    if value.starts_with('-') || value.starts_with("--") {
      only_flags_raw.push(value.to_owned());
    } else {
      commands_to_parse.push(value.to_owned())
    }
  });

  let parsed_flags = parse_flags(&only_flags_raw);

  let mut flags = HashMap::new();

  for (key, value) in parsed_flags {
    flags.insert(key, value);
  }

  Argv { commands: commands_to_parse, flags }
}
