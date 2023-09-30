#[test]
fn test_flags() {
  use crate::run::resolver::{resolve_command, Action};
  use crate::Argv;
  use std::collections::HashMap;

  let action = resolve_command(&Argv::from("command subcommands --version"), &[]);
  assert_eq!(action, Action::ShowVersion);

  let action = resolve_command(&Argv::from("command subcommands --help"), &[]);
  assert_eq!(action, Action::ShowHelp(HashMap::new()));
}
