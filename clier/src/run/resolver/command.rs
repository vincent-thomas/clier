use std::collections::HashMap;

use crate::{
  builder::{RCommand, RunnableCommand},
  hooks::use_flag,
  prelude::*,
  Argv,
};

pub enum FlagsAction {
  ShowHelp,
  ShowVersion,
  Nothing,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Action {
  RunCommand(String, RunnableCommand),
  ShowHelp(HashMap<String, RunnableCommand>),
  ShowVersion,
}
fn global_flags(argv: &Argv) -> FlagsAction {
  let is_version = use_flag("version", Some('v'), argv).try_into().unwrap_or(false);
  let is_help = use_flag("help", Some('h'), argv).try_into().unwrap_or(false);
  if is_version {
    FlagsAction::ShowVersion
  } else if is_help {
    FlagsAction::ShowHelp
  } else {
    FlagsAction::Nothing
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

pub(crate) fn command_fetcher(
  argv: &Argv,
  commands: HashMap<String, RunnableCommand>,
) -> (Vec<String>, Option<RunnableCommand>) {
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
  (valid_args, command.cloned())
}

pub(crate) fn resolve_command(argv: &Argv, registered_commands: &[RCommand]) -> Action {
  let commands = format_commands(registered_commands);
  match global_flags(argv) {
    FlagsAction::ShowHelp => return Action::ShowHelp(commands),
    FlagsAction::ShowVersion => return Action::ShowVersion,
    FlagsAction::Nothing => {}
  };

  let (valid_args, command) = command_fetcher(argv, commands.clone());

  if let Some(command_to_run) = command {
    Action::RunCommand(valid_args.join("."), command_to_run)
  } else {
    Action::ShowHelp(commands)
  }
}
