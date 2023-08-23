

use crate::command::Command;

use crate::CliApp;
use crate::ProgramOptions;
use crate::Spacing;


pub struct Cli {
  program_options: Option<ProgramOptions>,
  commands: Option<Vec<Command>>,
  args: Vec<String>
}

impl Cli {
  pub fn new(args: Vec<String>) -> Self {
    Cli {
      program_options: None,
      commands: None,
      args
    }
  }

  pub fn meta(mut self, program_options: ProgramOptions) -> Self {
    self.program_options = Some(program_options);
    self
  }


  pub fn build(self) -> CliApp {
    CliApp {
      spacing: Spacing::Dot,
      app_options: self.program_options.expect("Program options are requires"),
      commands: self.commands.expect("Commands are required"),
      args: self.args 
    }
  }
}

pub trait CommandBuilder {
  fn commands(self, commands: Vec<Command>) -> Self;
  fn command(self, command: Command) -> Self;
}

impl CommandBuilder for Cli {
  fn commands(mut self, commands: Vec<Command>) -> Self {
    self.commands = Some(commands);
    self
  }

  fn command(mut self, command: Command) -> Self {
    let mut commands = self.commands.unwrap_or(vec![]);
    commands.push(command);
    self.commands = Some(commands);
    self
  }
}
