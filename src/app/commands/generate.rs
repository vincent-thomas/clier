use std::fs;
use std::io::ErrorKind;

use crate::{
  app::config_parser::Config,
  builder::{CmdArgs, RCommand},
};
use clier::display::Displayer::*;
use clier::hooks::use_flag;

use crate::app::generators::CommandGenerator;

pub fn generate_command() -> RCommand {
  RCommand::new("generate", "Generates parts of program", command).usage("generate [--flags=value]")
}

fn command(args: CmdArgs) -> i32 {
  let dry_run = use_flag("dry-run", None, &args.args.flags).try_into().unwrap_or(false);

  if dry_run {
    Info.write("Dry run is enabled");
  }

  let type_ = match args.args.commands.get(0) {
    Some(value) => value,
    None => {
      eprintln!("Command not found");
      std::process::exit(1);
    }
  };
  if type_ == "command" || type_ == "c" {
    let config = Config::get();

    if let Err(err) = fs::create_dir(&config.command_dir) {
      if err.kind() != ErrorKind::AlreadyExists {
        println!("Unknown error: {}", err.kind());
        std::process::exit(1);
      }
    }

    let command_name = match use_flag("name", Some('n'), &args.args.flags).try_into() {
      Ok(value) => value,
      Err(_) => {
        let may_name = args.args.commands.get(1);
        if may_name.is_none() {
          Error.write_err("Flag: 'name', is required");
          std::process::exit(1);
        }
        may_name.unwrap().clone()
      }
    };
    let description =
      use_flag("desc", Some('d'), &args.args.flags).try_into().unwrap_or("todo...".to_string());
    let force = use_flag("force", Some('f'), &args.args.flags).try_into().unwrap_or(false);
    if force {
      Info.write("Force flag is enabled");
    }
    let file_writing =
      CommandGenerator::generate(config.clone(), &command_name, description, force, dry_run);

    match file_writing {
      Ok(_) => {
        Info.write(format!("Command generated at: {}/{}.rs", &config.command_dir, &command_name));

        Info.write("Import the file in main.rs to use the command");
        0
      }
      Err(err) => {
        Error.write_err(err.get_ref().expect("Unknown error").to_string());
        1
      }
    }
  } else {
    let msg = format!("Unknown command: {}", type_);
    Error.write_err(msg);
    1
  }
}
