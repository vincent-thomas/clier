// use clier::hooks::use_flags;

use clier::hooks::use_flags;

use crate::app::generators::ProjectGenerator;
use crate::builder::{CmdArgs, RCommand};

const NAME: &str = "new";
const DESCRIPTION: &str = "todo...";

pub fn new_command() -> RCommand {
  RCommand::new(NAME, DESCRIPTION, command)
    .flag("name", Some('n'), "Name of the project")
    .flag("desc", Some('d'), "Description")
    .subcommand("testing", "test", Some("test"), command)
    .subcommand("testtestsdfs", "test", Some("test"), command)
}

fn command(args: CmdArgs) -> i32 {
  println!("{args:?}");
  let flags = use_flags(&args);
  ProjectGenerator::generate(flags.get("name").unwrap(), flags.get("desc").unwrap());
  0
}
