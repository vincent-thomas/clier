use super::{AlreadyHasMeta, MissingMeta};
use crate::builder::{CmdArgs, Handler, RCommand};
use crate::resolver::{flag_resolver, resolve_command, Action};
use crate::Clier;
use crate::{prelude::*, Argv};
mod help;
use help::help;
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
  /// Usage examples. Used for clearance in help command
  pub usage: Option<String>,
  /// Version
  pub version: String,
}

/// Trait Runnable
pub trait Runnable {
  /// Add Command to Self.
  fn command(self, cmd: RCommand) -> Result<Self, Error>
  where
    Self: Sized;
  /// Generate root command where no arguments is passed.
  fn root(self, description: &str, handler: Handler) -> Result<Self, Error>
  where
    Self: Sized;
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
  fn command(mut self, cmd: RCommand) -> Result<Self, Error> {
    if cmd.name.contains('.') {
      Err(Error::InvalidFormat(cmd.name))
      // panic!(
      //   "{:?}",
      //   crate::prelude::Error::InvalidFormat(String::from("'name' can't contain '.'",))
      // );
    } else {
      self.registered_commands.push(cmd);
      Ok(self)
    }
  }

  fn root(self, description: &str, handler: Handler) -> Result<Self, Error> {
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
    let result = resolve_command(&self.args, &self.registered_commands);

    match result {
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
            let commands = if name.split('.').count() == 1 {
              self.args.commands
            } else {
              self.args.commands[name.split('.').count()..].to_vec()
            };

            let exit_code = (command.handler)(CmdArgs {
              args: Argv { flags: self.args.flags, commands },
              registered_flags: flags,
            })
            .into();
            Ok(exit_code)
          }
          Err(flag) => {
            println!("Flag not found: {flag}");
            Ok(1.into())
          }
        }
      }
    }
  }
}
