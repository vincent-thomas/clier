pub mod builder;
pub mod conf;
pub mod command;
mod core;

use crate::core::hooks::{core_use_flag, core_use_help};

use command::Command;
use conf::{ProgramOptions, Spacing};
#[cfg(feature = "hooks")]
pub mod hooks;
mod types;
#[derive(Debug, Clone)]
pub struct CliApp {
  app_options: ProgramOptions,
  spacing: Spacing,
  commands: Vec<Command>,
  args: Vec<String>,
}

impl CliApp {
  pub fn spacing(mut self, spacing: Spacing) -> Self {
    self.spacing = spacing;
    self
  }
}

#[derive(Debug)]
struct Formatting {}

#[derive(Debug)]
enum Error {
  FormattingError(Formatting)
}

impl CliApp {

  fn remove_flags(self, args: Vec<String>) -> Vec<String> {
    args.iter().filter(|value| !value.starts_with("-")).map(|value| value.to_string()).collect::<Vec<String>>()
  }


  fn format_commands(self) -> Result<Vec<String>, Error> {
    let mut args = self.clone().args;
    let maybe_ex = args.get(0).unwrap();
    let is_ex = std::path::Path::exists(std::path::Path::new(maybe_ex));
    if is_ex {
      args.remove(0);
    }

    let seperator = match self.spacing {
      Spacing::Dots => ':',
      Spacing::Space => ' ',
      Spacing::Dot => '.',
      Spacing::Custom(c) => c
    }.to_string();

    let result = self.clone().remove_flags(args).join(" ");
    let commands = result.split(seperator.as_str()).map(String::from).collect::<Vec<String>>();

    for item in &commands {
      if item.contains(" ") {
        return Err(Error::FormattingError(Formatting {}));
      }
    }
    return Ok(commands);
  }

  fn format_flags(self) -> types::FormattedFlagsOutput {
    self.args.iter().filter(|value| {
      value.starts_with("--")
    }).map(|flag| {flag.replace("--", "")}).map(|value| {
      let is_splittable = value.contains("=");

      if !is_splittable {
        return (value.to_string(), "true".to_string());
      }
      let iter = value.split("=").into_iter().collect::<Vec<&str>>();
      (iter.get(0).unwrap().to_string(), iter.get(1).unwrap().to_string())
    }).collect::<Vec<(String, String)>>()
  }

  pub fn run(self) {
    let commands = self.clone().format_commands().unwrap();
    let flags = self.clone().format_flags();
    let is_help = core_use_flag("help", &flags).value.unwrap_or("false".to_string());
    let mut the_command: Option<Command> = None;
    self.clone().commands.iter().for_each(|command| {
      if commands.get(0).unwrap() == command.name {
        the_command = Some(command.clone());
      }
    });
    if is_help == "true" || the_command.is_none() {
      core_use_help(self.clone().commands, self.clone().app_options);
    }
    let options = conf::CommandArgs {
      commands,
      flags,
      conf: self.clone().app_options
    };
    (the_command.unwrap().handler)(options);
  }
}
