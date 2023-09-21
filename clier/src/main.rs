mod app;

use app::commands::generate::generate_command;
use app::commands::new::new_command;
use clier::help::help;
use clier::run::Runnable;
use clier::{CliMeta, Clier, ExitCode};
use clier_builder as builder;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() -> clier::ExitCode {
  let meta = CliMeta {
    name: NAME.to_string(),
    description: DESCRIPTION.to_string(),
    version: VERSION.to_string(),
    usage: None,
  };

  let cli = Clier::parse().meta(&meta).add_command(generate_command()).add_command(new_command());
  match cli.clone().run() {
    Ok(value) => value,
    Err(value) => match value {
      clier::error::Error::CommandNotFound(command) => {
        if command.is_empty() {
          help(cli.get_commands().as_slice(), &cli.args.commands, meta);
        } else {
          println!("Command not found: {}", command);
        }
        ExitCode(0)
      }
      _ => ExitCode(0),
    },
  }
}
