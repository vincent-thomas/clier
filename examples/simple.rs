
use std::env;

use clier::{CliBuilder, Command, Runnable, hooks::{use_flag, OutputCheck}};

fn main() {

  let clier_builder = CliBuilder::new()
    .meta("things", "This is the description", "1.0.0", "This is the usage");

  let app = clier_builder
  .command(Command {
    name: "test",
    description: "detta är hur man gör",
    handler: |value| {
      let test = use_flag("test", &value.flags);
      println!("this is the command {} and is {}", test.value.clone().unwrap_or("false".to_string()), &test.is_true());
    },
    help_string: Some("This is the help"),
    usage: None
  })
  .command(Command {
    name: "another-command",
    description: "en till",
    handler: |value| {
      let test = use_flag("test-2", &value.flags);
      println!("this is the command {} and is {}", test.value.clone().unwrap(), &test.is_true());
    },
    help_string: Some("This is the more help"),
    usage: None
  })
    .build(env::args().collect());

  app.run();
}