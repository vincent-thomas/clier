mod builder;
use crate::command::Command;
use self::format::Args;
mod runner;
pub use runner::*;
mod help;
pub use help::*;

#[derive(Debug, Clone, Copy)]
pub enum Spacing {
  Dots,
  Space,
  Dot,
  Custom(char)
}
pub(crate) mod format;



#[derive(Debug, Clone)]
pub struct App {
  pub name: String,
  pub description: String,
  pub usage: Option<String>,
  pub version: String,
}

#[derive(Debug, Clone)]
pub struct CliBuilder {
  name: Option<String>,
  raw_args: Option<Vec<String>>,
  version: Option<String>,
  description: Option<String>,
  usage: Option<String>,
  commands: Option<Vec<Command>>,
}

#[derive(Debug)]
pub struct Cli {
  pub options: App,
  pub commands: Vec<Command>,
  pub args: Args
}