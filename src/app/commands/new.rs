use clier::{
  command::{CmdArgs, Command},
  hooks::{use_flags, Flag},
};

use crate::app::generators::ProjectGenerator;

const NAME: &str = "new";
const DESCRIPTION: &str = "todo...";

pub fn new_command() -> Command {
  Command::new(NAME, DESCRIPTION, command).flags(vec![
    Flag::new("name", "Name of the project".to_string()).short('n'),
    Flag::new("desc", "Description".to_string()).short('d'),
  ])
}

fn command(args: CmdArgs) -> i32 {
  let flags = use_flags(&args);
  ProjectGenerator::generate(flags.get("name").unwrap(), flags.get("desc").unwrap());
  0
}
