use crate::command::{CmdArgs, Command, Handler};
use crate::error::Error;
use crate::help::help;
use crate::prelude::CResult;
use crate::{CliMeta, Clier, ExitCode};

use super::format::match_command;
use super::utils::{format_validate_reg_flags, global_flags};
use super::{Meta, MissingMeta};

pub trait Runnable {
  fn add_command(self, cmd: Command) -> Self;
  fn root(self, description: &str, handler: Handler) -> Self;
  fn commands(self, cmd: Vec<Command>) -> Self;
  fn run(self) -> Result<ExitCode, Error>;
}

impl Clier<MissingMeta> {
  pub fn meta(self, meta: CliMeta) -> Clier<Meta> {
    Clier { options: Meta(meta), args: self.args, registered_commands: self.registered_commands }
  }
}
impl Runnable for Clier<Meta> {
  fn add_command(mut self, cmd: Command) -> Self {
    if cmd.name.contains('.') {
      panic!("{:?}", Error::InvalidFormat(String::from("'name' can't contain '.'",)));
    }
    self.registered_commands.push(cmd);
    self
  }

  // Impl root
  fn root(self, description: &str, handler: Handler) -> Self {
    let options = self.clone().options.0;
    let mut root_command = Command::new("root", description, handler);

    if let Some(usage) = options.usage {
      root_command = root_command.clone().usage(usage.as_str());
    }
    self.add_command(root_command)
  }

  fn commands(mut self, cmd: Vec<Command>) -> Self {
    self.registered_commands = cmd;
    self
  }

  fn run(self) -> CResult<ExitCode> {
    match global_flags(
      &self.args.flags,
      &self.args.commands,
      &self.registered_commands,
      self.clone().options.0,
    ) {
      true => return CResult::Ok(ExitCode(0)),
      false => {}
    };

    let command_to_run = match_command(&self.registered_commands, &self.args.commands);

    if let Some(command) = command_to_run {
      let reg_flags = match format_validate_reg_flags(&self.args.flags, &command) {
        CResult::Ok(value) => value,
        CResult::Err(value) => {
          help(&self.registered_commands, &self.args.commands, self.options.0);
          println!("\n{}", value);
          std::process::exit(1);
        }
      };

      let args = CmdArgs {
        commands: self.clone().args.commands,
        flags: self.clone().args.flags,
        registered_flags: reg_flags,
      };

      let exit_code = (command.handler)(args);

      Ok(ExitCode(exit_code))
    } else {
      Err(Error::CommandNotFound(self.clone().args.commands.join(" ")))
    }
  }
}
