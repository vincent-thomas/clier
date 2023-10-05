mod help;
mod impl_runnable;
mod resolver;

use super::{AlreadyHasMeta, MissingMeta};
use crate::builder::{/* CmdArgs, */ Handler, RCommand};
use crate::prelude::*;
use crate::Clier;
use std::process::Termination;

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
