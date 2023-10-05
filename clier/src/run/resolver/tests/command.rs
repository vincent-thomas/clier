#[test]
fn test_resolve_commands() {
  use crate::run::resolver::resolve_command;

  use crate::{
    builder::{RCommand, RunnableCommand},
    run::resolver::Action,
    Argv,
  };
  // Måste vara funktion p.g.a man jämför minnesadresser till funktionen.
  let handler = |_args| 0;

  let action = resolve_command(
    &Argv::from("command subcommand"),
    &[RCommand {
      name: "command".to_string(),
      description: "description".to_string(),
      handler,
      // usage: Some("test".into()),
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
        // usage: Some("test".into())
      }
    )
  );

  let action = resolve_command(
    &Argv::from("command subcommand"),
    &[
      RCommand {
        name: "command".to_string(),
        description: "description".to_string(),
        handler,
        // usage: None,
        flags: None,
        children: None,
      },
      RCommand {
        name: "command.subcommand".to_string(),
        description: "description".to_string(),
        handler,
        // usage: None,
        flags: None,
        children: None,
      },
    ],
  );

  assert_eq!(
    action,
    Action::RunCommand(
      "command.subcommand".to_string(),
      RunnableCommand {
        description: "description".to_string(),
        handler,
        flags: None, /*usage: None*/
      }
    )
  );
}
