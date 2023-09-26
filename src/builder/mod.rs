mod command;
mod flag;

pub use flag::*;

pub use command::*;

use crate::Argv;

/// Handler
pub type Handler = fn(args: CmdArgs) -> i32;

/// The CmdArgs struct that is passed to all command handlers.
#[derive(Debug, Clone)]
pub struct CmdArgs {
  /// struct 'Argv' contains parsed flags and commands.
  pub args: Argv,
  /// Registered flags for the command by the struct 'Command::flag'.
  pub registered_flags: Vec<(String, Flag)>,
}
