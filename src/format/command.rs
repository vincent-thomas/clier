use crate::command::Command;
use std::collections::HashMap;

pub(crate) fn matcher(registered_commands: &[Command], commands: &[String]) -> Option<Command> {
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
