use crate::flags_argv::transform_flags_argv;
use crate::{commands_argv, Argv};
use commands_argv::transform_command_argv;

pub fn transform_vargs(args: &[String]) -> Argv {
  let flags = transform_flags_argv(args);
  let commands = transform_command_argv(args);

  Argv { commands, flags }
}
