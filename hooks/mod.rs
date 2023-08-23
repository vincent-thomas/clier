use crate::core::hooks::{core_use_flag, core_use_help};
use crate::types::Flag;
use crate::command::Command;
use crate::conf::ProgramOptions;

pub fn use_flag(name: &'static str, args: &[(String, String)]) -> Flag {
  core_use_flag(name, args)
}

pub fn use_help(commands: Vec<Command>, options: ProgramOptions) {
  core_use_help(commands, options)
}
