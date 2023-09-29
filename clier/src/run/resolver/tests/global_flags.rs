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
