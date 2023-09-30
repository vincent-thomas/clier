mod help;
mod resolver;

use super::{AlreadyHasMeta, MissingMeta};
use crate::builder::{CmdArgs, Handler, RCommand};
use crate::prelude::*;
use crate::Clier;
use help::help;
use resolver::{flag_resolver, resolve_command, Action};
use std::process::Termination;

use console::Term;

/// ExitCode is a wrapper around i32 to implement Termination trait
#[derive(Debug, Clone)]
pub struct ExitCode(pub i32);

impl Termination for ExitCode {
  fn report(self) -> std::process::ExitCode {
    std::process::exit(self.0)
  }
}

impl From<i32> for ExitCode {
  fn from(code: i32) -> Self {
    Self(code)
  }
}

/// Meta information required for help and version commands
#[derive(Debug, Clone, Default)]
pub struct Meta {
  /// Name of the binary application
  pub name: String,
  /// Description of the binary application
  pub description: String,
  /// Version
  pub version: String,
  /// Usage examples. Used for clearance in help command
  pub usage: Option<String>,
}

/// Trait Runnable
pub trait Runnable {
  /// Add Command to Self.
  fn command(self, cmd: RCommand) -> Self;
  /// Generate root command where no arguments is passed.
  fn root(self, description: &str, handler: Handler) -> Self;
  /// Add multiple commands to Self. Overrides all previous commands.
  fn commands(self, cmd: Vec<RCommand>) -> Self;
  /// Get all registered commands.nd>) -> Self;
  // fn get_commands(&self) -> HashMap<String, RunnableCommand>;
  /// Runs all commands and returns [ExitCode].
  fn run(self) -> Result<ExitCode, Error>;
}

impl Meta {
  /// Create new [Meta]
  pub fn new(
    name: impl Into<String>,
    description: impl Into<String>,
    version: impl Into<String>,
  ) -> Self {
    Self {
      name: name.into(),
      description: description.into(),
      usage: None,
      version: version.into(),
    }
  }
  /// Add usage to [Meta]
  pub fn usage(mut self, usage: &str) -> Self {
    self.usage = Some(usage.to_string());
    self
  }
}

impl Clier<MissingMeta> {
  /// Add [Meta] to Clier.
  pub fn meta(self, meta: &Meta) -> Clier<AlreadyHasMeta> {
    Clier {
      options: AlreadyHasMeta(meta.clone()),
      args: self.args,
      registered_commands: self.registered_commands,
    }
  }
}

impl Runnable for Clier<AlreadyHasMeta> {
  fn command(mut self, cmd: RCommand) -> Self {
    self.registered_commands.push(cmd);
    self
  }

  fn root(self, description: &str, handler: Handler) -> Self {
    let options = self.clone().options.0;
    let mut root_command = RCommand::new("root", description, handler);

    if let Some(usage) = options.usage {
      root_command = root_command.usage(usage.as_str());
    }
    self.command(root_command)
  }

  fn commands(mut self, cmd: Vec<RCommand>) -> Self {
    self.registered_commands = cmd;
    self
  }

  fn run(self) -> Result<ExitCode, Error> {
    let what_to_do = resolve_command(&self.args, &self.registered_commands);

    match what_to_do {
      Action::ShowHelp(commands) => {
        help(&commands, &self.args.commands, self.clone().options.0);
        Ok(0.into())
      }
      Action::ShowVersion => {
        let term = Term::stdout();
        let _ = term.write_line(&f!("v{}", self.clone().options.0.version));
        Ok(0.into())
      }
      Action::RunCommand(name, command) => {
        let registered_flags = flag_resolver(&command.flags.unwrap_or(vec![]), &self.args.flags);
        match registered_flags {
          Ok(flags) => {
            let mut commands = self.args.commands.clone();
            for _ in 0..name.split('.').count() {
              commands.remove(0);
            }

            let mut args_default = self.args.clone();
            args_default.commands = commands;
            args_default.flags = self.args.flags;

            let exit_code =
              (command.handler)(CmdArgs { args: args_default, registered_flags: flags }).into();
            Ok(exit_code)
          }
          Err(flag) => {
            eprintln!("Flag not found: {flag}");
            Ok(1.into())
          }
        }
      }
    }
  }
}
