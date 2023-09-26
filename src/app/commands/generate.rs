use std::fs;
use std::io::ErrorKind;

use crate::{
  app::generators::Config,
  builder::{CmdArgs, RCommand},
};
use clier::hooks::{use_flag, use_flags};

use crate::app::generators::CommandGenerator;

pub fn generate_command() -> RCommand {
  RCommand::new("generate", "Generates parts of program", command)
    .usage("generate [--flags=value]")
    .flag("type", Some('t'), "type of thing to generate")
}

fn command(args: CmdArgs) -> i32 {
  println!("{:?}", args);
  let flags = use_flags(&args);
  let t = flags.get("type").unwrap();

  if t.as_str() == "command" {
    let config = Config::get();

    if let Err(err) = fs::create_dir(&config.command_dir) {
      if err.kind() != ErrorKind::AlreadyExists {
        println!("Unknown error: {}", err.kind());
        std::process::exit(1);
      }
    }

    let command_name = match use_flag("name", Some('n'), &args.args.flags).0 {
      Some(value) => value,
      None => {
        eprintln!("flag name, is required");
        std::process::exit(1);
      }
    };
    let description =
      use_flag("desc", Some('d'), &args.args.flags).0.unwrap_or("todo...".to_string());
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
