use crate::command::Command;
use crate::format::transform_vargs;
use std::env::args;
use std::process::Termination;

#[derive(Debug, Clone, Default)]
pub struct Args {
  pub commands: Vec<String>,
  pub flags: Vec<(String, String)>,
}

#[derive(Debug, Clone, Default)]
pub struct CliMeta {
  pub name: String,
  pub description: String,
  pub usage: Option<String>,
  pub version: String,
}

// region:    --- Meta States
#[derive(Debug, Default, Clone)]
pub struct MissingMeta;
#[derive(Debug, Default, Clone)]
pub struct Meta(pub(crate) CliMeta);
// endregion: --- Meta States

#[derive(Debug, Clone, Default)]
pub struct Clier<T> {
  pub(crate) options: T,
  pub(crate) registered_commands: Vec<Command>,
  pub args: Args,
}

#[derive(Debug, Clone)]
pub struct ExitCode(pub i32);

impl Termination for ExitCode {
  fn report(self) -> std::process::ExitCode {
    std::process::exit(self.0)
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
