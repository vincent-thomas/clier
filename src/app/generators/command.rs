use super::Config;
use std::{fs::File, io::Write};

pub struct CommandGenerator;

impl CommandGenerator {
  pub fn generate(
    config: Config,
    name: impl Into<String>,
    description: impl Into<String>,
  ) -> Result<(), std::io::Error> {
    let name = name.into();
    let description = description.into();

    let file_path = format!("{}/{}.rs", &config.command_dir, name);
    let mut file = File::create(file_path).unwrap();
    let file_writing = file.write_all(
      format!(
        "use clier::command::{{CmdArgs, Command}};

const NAME: &'static str = \"{name}\";
const DESCRIPTION: &'static str = \"{description}\";

pub fn {name}_command() -> Command {{
  Command::new(NAME, DESCRIPTION, command)
}}

fn command(args: CmdArgs) -> i32 {{
    println!(\"Hello World\");
    0
}}
",
      )
      .as_bytes(),
    );

    file_writing
  }
}
