use std::fs;
use std::io::ErrorKind;

use clier::command::{CmdArgs, Command};
use clier::hooks::{use_flag, use_flags, Flag};

use crate::app::generators::{get_config, CommandGenerator};

pub fn generate_command() -> Command {
  Command::new("generate", "Generates parts of program", command)
    .usage("generate [--flags=value]")
    .flags(vec![Flag::new("type", "type of thing to generate".to_string()).short('t')])
}

fn command(args: CmdArgs) -> i32 {
  let flags = use_flags(&args);
  let t = flags.get("type").unwrap().clone();

  if t.as_str() == "command" {
    let config = get_config();

    if let Err(err) = fs::create_dir(&config.command_dir) {
      if err.kind() != ErrorKind::AlreadyExists {
        println!("Unknown error: {}", err.kind());
        std::process::exit(1);
      }
    }

    let command_name = match use_flag("name", Some('n'), &args.args.flags).value {
      Some(value) => value,
      None => {
        eprintln!("flag name, is required");
        std::process::exit(1);
      }
    };
    let description =
      use_flag("desc", Some('d'), &args.args.flags).value.unwrap_or("todo...".to_string());
    let file_writing = CommandGenerator::generate(config.clone(), &command_name, description);

    match file_writing {
      Ok(_) => {
        println!(
          "command {} written at {}/{}.rs",
          command_name, &config.command_dir, &command_name
        );
        println!("Note: This tool doesn't add the command to the main.rs file, coming soon...");
      }
      Err(err) => println!("Unknown Error: {}", err),
    }
  } else if t.as_str() == "flag" {
    println!("Generating flags");
  };
  0
}
