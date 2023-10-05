use clier::display::Displayer::Error;
use clier::hooks::{use_flag, FlagError};

use crate::generators::ProjectGenerator;
use clier::builder::{CmdArgs, RCommand};

const NAME: &str = "new";
const DESCRIPTION: &str = "Generate a new project in a subdir";

pub fn new_command() -> RCommand {
  RCommand::new(NAME, DESCRIPTION, command)
}

fn command(args: CmdArgs) -> i32 {
  let project_name = args.args.commands.get(0).unwrap_or_else(|| {
    Error.write("Project name is required");
    std::process::exit(1);
  });

  let desc: Result<String, FlagError> = use_flag("desc", Some('d'), &args.args).try_into();
  let desc = desc.unwrap_or("todo...".to_string());
  ProjectGenerator::generate(project_name, desc);
  0
}
