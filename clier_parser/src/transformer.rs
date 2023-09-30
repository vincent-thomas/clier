use crate::flags_argv::transform_flags_argv;
use crate::{commands_argv, Argv};
use commands_argv::transform_command_argv;

// Behövs inte testa, gör det i lib.rs
pub fn transform_vargs(args: &[String]) -> Argv {
  let mut encountered_dash_dash = false;
  let mut after_double_dash = "".to_string();
  let args: Vec<String> = args
    .iter()
    .filter(|arg| match (arg.as_str() == "--", encountered_dash_dash) {
      (true, _) => {
        encountered_dash_dash = true;
        false
      }
      (false, true) => {
        after_double_dash.push_str(format!(" {arg}").as_str());
        false
      }
      _ => true,
    })
    .cloned()
    .collect();

  if !after_double_dash.is_empty() {
    after_double_dash.remove(0);
  }

  let flags = transform_flags_argv(&args);
  let commands = transform_command_argv(&args);

  Argv { commands, flags, after_double_dash }
}

// Testar redan commands. Behöver inte det nu
#[test]
fn test_transform_vargs() {
  use std::collections::HashMap;
  let result = transform_vargs(
    "command subcommand --name=test --value=false -fe=t -vt value -ui --test1 --no-fdsafsa test"
      .split(' ')
      .map(|v| v.to_string())
      .collect::<Vec<String>>()
      .as_slice(),
  );
  let mut hash = HashMap::new();
  hash.insert("name".to_string(), "test".to_string());
  hash.insert("value".to_string(), "false".to_string());
  hash.insert("fdsafsa".to_string(), "false".to_string());
  hash.insert("v".to_string(), "true".to_string());
  hash.insert("t".to_string(), "value".to_string());
  hash.insert("f".to_string(), "true".to_string());
  hash.insert("e".to_string(), "t".to_string());
  hash.insert("u".to_string(), "true".to_string());
  hash.insert("i".to_string(), "true".to_string());
  hash.insert("test1".to_string(), "true".to_string());

  for flag in hash.clone() {
    let left = result.flags.get(&flag.0);
    let right = Some(&flag.1);
    if left != right {
      panic!("Not right flag: {}: {}={:?}", flag.0, flag.1, left);
    }
  }
}
