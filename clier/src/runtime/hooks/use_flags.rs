use std::collections::HashMap;

use crate::builder::CmdArgs;

/// Use all registered commands
pub fn use_flags(args: &CmdArgs) -> HashMap<String, String> {
  let mut flags = HashMap::new();

  args
    .clone()
    .registered_flags
    .into_iter()
    .map(|(flag_name, flag_value)| (flag_name, flag_value.value.unwrap()))
    .for_each(|value| {
      flags.insert(value.0, value.1);
    });

  flags
}

#[test]
fn test_use_flags() {
  use crate::builder::Flag;
  use crate::Argv;
  let registered_flags = vec![
    (
      "name".to_string(),
      Flag {
        name: "name".to_string(),
        description: "name".to_string(),
        value: Some("John".to_string()),
      },
    ),
    (
      "age".to_string(),
      Flag {
        name: "age".to_string(),
        description: "age".to_string(),
        value: Some("20".to_string()),
      },
    ),
  ]
  .into_iter()
  .collect();

  let args = CmdArgs { registered_flags, args: Argv::from("") };

  let result = use_flags(&args);
  assert_eq!(result.len(), 2);
}
