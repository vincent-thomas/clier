use super::{AlreadyHasMeta, MissingMeta};
use crate::error::Error;
use crate::format::{command, flags};
use crate::help::help;
use crate::hooks::use_flag;
use crate::prelude::*;
use crate::{CliMeta, Clier, ExitCode};

use crate::builder::{CmdArgs, Command, Handler};
use clier_parser::Argv;
use console::Term;

pub trait Runnable {
  fn add_command(self, cmd: Command) -> Self;
  fn root(self, description: &str, handler: Handler) -> Self;
  fn commands(self, cmd: Vec<Command>) -> Self;
  fn get_commands(&self) -> Vec<Command>;
  fn run(self) -> Result<ExitCode>;
}

impl Clier<MissingMeta> {
  pub fn meta(self, meta: &CliMeta) -> Clier<AlreadyHasMeta> {
    Clier {
      options: AlreadyHasMeta(meta.to_owned()),
      args: self.args,
      registered_commands: self.registered_commands,
    }
  }
}

fn global_flags(argv: &Argv, registered_commands: &[Command], meta: CliMeta) -> bool {
  let is_version = use_flag("version", Some('v'), &argv.flags).try_into().unwrap_or(false);
  let is_help = use_flag("help", Some('h'), &argv.flags).try_into().unwrap_or(false);
  match (is_version, is_help) {
    (true, _) => {
      let term = Term::stdout();
      let _ = term.write_line(&f!("v{}", meta.version));
      is_version
    }
    (_, true) => {
      help(registered_commands, &argv.commands, meta);
      is_help
    }
    (_, _) => false,
  }
}

impl Runnable for Clier<AlreadyHasMeta> {
  fn get_commands(&self) -> Vec<Command> {
    self.registered_commands.clone()
  }
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
      root_command = root_command.usage(usage.as_str());
    }
    self.add_command(root_command)
  }

  fn commands(mut self, cmd: Vec<Command>) -> Self {
    self.registered_commands = cmd;
    self
  }

  fn run(self) -> Result<ExitCode> {
    if global_flags(&self.args, &self.registered_commands, self.clone().options.0) {
      return Result::Ok(ExitCode(0));
    }

    let Some(command) = command::matcher(&self.registered_commands, &self.args.commands) else {
      return Err(Error::CommandNotFound(self.args.commands.join(" ")));
    };

    let registered_flags = match flags::format_registered(&self.args.flags, &command) {
      Result::Ok(value) => value,
      Result::Err(value) => {
        help(&self.registered_commands, &self.args.commands, self.options.0);
        return Result::Err(value);
      }
    };

    let args = CmdArgs { args: self.args, registered_flags };
    let exit_code = (command.handler)(args);

    Ok(ExitCode(exit_code))
  }
}
