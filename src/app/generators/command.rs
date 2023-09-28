use std::{fs::File, io::Write, path::Path};

use crate::app::config_parser::Config;

pub struct CommandGenerator;

impl CommandGenerator {
  pub fn generate(
    config: Config,
    name: impl Into<String>,
    description: impl Into<String>,
    force: bool,
    dry_run: bool,
  ) -> Result<(), std::io::Error> {
    let name = name.into();
    let description = description.into();

    let file_path = format!("{}/{}.rs", &config.command_dir, name);
    let does_exist = Path::new(&file_path).exists();
    if does_exist && !force {
      return Err(std::io::Error::new(
        std::io::ErrorKind::AlreadyExists,
        format!("File already exists: {}", file_path),
      ));
    }
    if !dry_run {
      let mut file = File::create(file_path).unwrap();

      let file_writing = file.write_all(
        format!(
          "use clier::builder::{{CmdArgs, RCommand}};

const NAME: &str = \"{name}\";
const DESCRIPTION: &str = \"{description}\";

pub fn {name}_command() -> Command {{
  RCommand::new(NAME, DESCRIPTION, command)
}}

fn command(args: CmdArgs) -> i32 {{
    println!(\"Hello World\");
    0
}}
",
        )
        .as_bytes(),
      );
      return file_writing;
    };
    Ok(())
  }
}
