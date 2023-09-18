use crate::command::Command;
use crate::prelude::Flags;
use crate::utils::transform_vargs;
use std::env::args;
use std::fmt::{Debug, Formatter};
use std::process::Termination;

#[derive(Debug, Clone, Default)]
pub struct Argv {
  pub commands: Vec<String>,
  pub flags: Flags,
}

#[derive(Debug, Clone, Default)]
pub struct CliMeta {
  pub name: String,
  pub description: String,
  pub usage: Option<String>,
  pub version: String,
}

// region: Meta States
#[derive(Debug, Default, Clone)]
pub struct MissingMeta;
#[derive(Debug, Default, Clone)]
pub struct AlreadyHasMeta(pub(crate) CliMeta);
// endregion: Meta States

#[derive(Clone, Default)]
pub struct Clier<T> {
  pub(crate) options: T,
  pub(crate) registered_commands: Vec<Command>,
  pub args: Argv,
}

#[derive(Debug, Clone)]
pub struct ExitCode(pub i32);

impl Termination for ExitCode {
  fn report(self) -> std::process::ExitCode {
    std::process::exit(self.0)
  }
}

impl<T: Debug> Debug for Clier<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Clier").field("args", &self.args).finish()
  }
}

impl Clier<MissingMeta> {
  pub fn parse() -> Clier<MissingMeta> {
    Clier {
      options: MissingMeta,
      registered_commands: vec![],
      args: transform_vargs(&args().collect::<Vec<String>>()[1..]),
    }
  }
}
