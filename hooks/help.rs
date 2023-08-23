use crate::core::hooks::core_use_help;
use crate::conf::ProgramOptions;
use crate::command::Command;

pub fn use_help(commands: Vec<Command>, options: ProgramOptions) {
  core_use_help(commands, options)
}
