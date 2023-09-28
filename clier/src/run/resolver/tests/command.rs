#[test]
fn test_resolve_commands() {
  use crate::run::resolver::resolve_command;
  use std::collections::HashMap;

  use crate::{
    builder::{RCommand, RunnableCommand},
    run::resolver::Action,
    Argv,
  };
  // Måste vara funktion p.g.a man jämför minnesadresser till funktionen.
  let handler = |_args| 0;

  let action = resolve_command(
    &Argv {
      commands: vec!["command".to_string(), "subcommands".to_string()],
      flags: HashMap::new(),
    },
    &[RCommand {
      name: "command".to_string(),
      description: "description".to_string(),
      handler,
      usage: Some("test".into()),
      flags: None,
      children: None,
    }],
  );

  assert_eq!(
    action,
    Action::RunCommand(
      "command".to_string(),
      RunnableCommand {
        description: "description".to_string(),
        handler,
        flags: None,
        usage: Some("test".into())
      }
    )
  );

  let action = resolve_command(
    &Argv {
      commands: vec!["command".to_string(), "subcommand".to_string()],
      flags: HashMap::new(),
    },
    &[
      RCommand {
        name: "command".to_string(),
        description: "description".to_string(),
        handler,
        usage: None,
        flags: None,
        children: None,
      },
      RCommand {
        name: "command.subcommand".to_string(),
        description: "description".to_string(),
        handler,
        usage: None,
        flags: None,
        children: None,
      },
    ],
  );

  assert_eq!(
    action,
    Action::RunCommand(
      "command.subcommand".to_string(),
      RunnableCommand { description: "description".to_string(), handler, flags: None, usage: None }
    )
  );
}
#[test]
fn test_flags() {
  use crate::run::resolver::{resolve_command, Action};
  use crate::Argv;
  use std::collections::HashMap;

  let action = resolve_command(
    &Argv {
      commands: vec!["command".to_string(), "subcommands".to_string()],
      flags: HashMap::from([
        ("version".to_string(), "true".to_string()),
        ("help".to_string(), "false".to_string()),
      ]),
    },
    &[],
  );

  assert_eq!(action, Action::ShowVersion);

  let action = resolve_command(
    &Argv {
      commands: vec!["command".to_string(), "subcommands".to_string()],
      flags: HashMap::from([
        ("version".to_string(), "false".to_string()),
        ("help".to_string(), "true".to_string()),
      ]),
    },
    &[],
  );

  assert_eq!(action, Action::ShowHelp(HashMap::new()));
}
