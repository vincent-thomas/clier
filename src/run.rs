use crate::command::{CmdArgs, Command, Handler};
use crate::error::Error;
use crate::help::help;
use crate::prelude::CResult;
use crate::{CliMeta, Clier, ExitCode};

use super::format::match_command;
use super::utils::{format_validate_reg_flags, global_flags};
use super::{AlreadyHasMeta, MissingMeta};

pub trait Runnable {
  fn add_command(self, cmd: Command) -> Self;
  fn root(self, description: &str, handler: Handler) -> Self;
  fn commands(self, cmd: Vec<Command>) -> Self;
  fn run(self) -> Result<ExitCode, Error>;
}

impl Clier<MissingMeta> {
  pub fn meta(self, meta: CliMeta) -> Clier<AlreadyHasMeta> {
    Clier {
      options: AlreadyHasMeta(meta),
      args: self.args,
      registered_commands: self.registered_commands,
    }
  }
}
impl Runnable for Clier<AlreadyHasMeta> {
  fn add_command(mut self, cmd: Command) -> Self {
    if cmd.name.contains('.') {
      panic!("{:?}", Error::InvalidFormat(String::from("'name' can't contain '.'",)));
    }
    self.registered_commands.push(cmd);
    self
  }

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
    if global_flags(&self.args, &self.registered_commands, self.clone().options.0) {
      return CResult::Ok(ExitCode(0));
    }

    let Some(command) = match_command(&self.registered_commands, &self.args.commands) else {
      return Err(Error::CommandNotFound(self.args.commands.join(" ")));
    };

    let registered_flags = match format_validate_reg_flags(&self.args.flags, &command) {
      CResult::Ok(value) => value,
      CResult::Err(value) => {
        help(&self.registered_commands, &self.args.commands, self.options.0);
        return CResult::Err(value);
      }
    };

    let args = CmdArgs { args: self.args, registered_flags };
    let exit_code = (command.handler)(args);

    Ok(ExitCode(exit_code))
  }
}
