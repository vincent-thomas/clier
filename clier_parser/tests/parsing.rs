use hashbrown::HashMap;

use clier_parser::Argv;

#[test]
fn argv_from() {
  let result = Argv::from(
    "command subcommand --name=test --value=false -fe=t -vt value -ui --ui --no-fdsafsa test",
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
  hash.insert("ui".to_string(), "true".to_string());

  for flag in hash.clone() {
    let left = result.flags.get(&flag.0);
    let right = Some(&flag.1);
    if left != right {
      panic!("Not right flag: {}: {}={:?}", flag.0, flag.1, left);
    }
  }

  let commands = ["command".to_string(), "subcommand".to_string()];
  for (i, command) in commands.iter().enumerate() {
    if result.commands[i] != *command {
      panic!("Not right command: {}", command);
    }
  }
}
