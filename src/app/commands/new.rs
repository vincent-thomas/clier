use clier::hooks::use_flags;

use crate::app::generators::ProjectGenerator;
use crate::builder::{CmdArgs, Command};

const NAME: &str = "new";
const DESCRIPTION: &str = "todo...";

pub fn new_command() -> Command {
  Command::new(NAME, DESCRIPTION, command).flag("name", Some('n'), "Name of the project").flag(
    "desc",
    Some('d'),
    "Description",
  )
}

fn command(args: CmdArgs) -> i32 {
  let flags = use_flags(&args);
  ProjectGenerator::generate(flags.get("name").unwrap(), flags.get("desc").unwrap());
  0
}
