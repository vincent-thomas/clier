use std::collections::HashMap;

use crate::{
  builder::{RCommand, RunnableCommand},
  hooks::use_flag,
  prelude::*,
  Argv,
};

pub enum Action1 {
  ShowHelp,
  ShowVersion,
  Nothing,
}

pub(crate) enum Action {
  RunCommand(String, RunnableCommand),
  ShowHelp(HashMap<String, RunnableCommand>),
  ShowVersion,
}
fn global_flags(argv: &Argv) -> Action1 {
  let is_version = use_flag("version", Some('v'), &argv.flags).try_into().unwrap_or(false);
  let is_help = use_flag("help", Some('h'), &argv.flags).try_into().unwrap_or(false);
  if is_version {
    Action1::ShowVersion
  } else if is_help {
    Action1::ShowHelp
  } else {
    Action1::Nothing
  }
}
fn format_commands(registered_commands: &[RCommand]) -> HashMap<String, RunnableCommand> {
  let commands_vec: Vec<(String, RunnableCommand)> = registered_commands
    .iter()
    .flat_map(|v| -> Vec<(String, RunnableCommand)> { transform(v.clone(), None) })
    .collect();
  fn transform(val: RCommand, prefix: Option<&str>) -> Vec<(String, RunnableCommand)> {
    let children = if let Some(children) = val.children {
      let mut ano = vec![];
      for item in children {
        ano.push(transform(item, Some(&val.name)));
      }
      ano
    } else {
      vec![]
    };
    let name =
      if let Some(real_prefix) = prefix { f!("{}.{}", real_prefix, val.name) } else { val.name };
    let mut to_return = vec![(
      name,
      RunnableCommand {
        handler: val.handler,
        usage: val.usage,
        flags: val.flags,
        description: val.description,
      },
    )];
    children.into_iter().flatten().for_each(|v| to_return.push(v));
    to_return
  }
  HashMap::from_iter(commands_vec)
}

pub(crate) fn resolve_command(argv: &Argv, registered_commands: &[RCommand]) -> Action {
  let commands = format_commands(registered_commands);
  match global_flags(argv) {
    Action1::ShowHelp => return Action::ShowHelp(commands),
    Action1::ShowVersion => return Action::ShowVersion,
    Action1::Nothing => {}
  };

  if let Some(command_to_run) = commands.get(argv.commands.join(".").as_str()) {
    return Action::RunCommand(argv.commands.join("."), command_to_run.clone());
  }

  let valid_args = argv
    .commands
    .iter()
    .enumerate()
    .filter(|(index, _)| {
      let actual_name = argv.commands[0..index + 1].join(".");
      commands.get(&actual_name).is_some()
    })
    .map(|v| v.1.clone())
    .collect::<Vec<String>>();

  let mut command = commands.get(&valid_args.join("."));

  if command.is_none() && valid_args.is_empty() {
    command = commands.get("root");
  }

  if let Some(command_to_run) = command {
    Action::RunCommand(valid_args.join("."), command_to_run.clone())
  } else {
    Action::ShowHelp(commands)
  }
}
