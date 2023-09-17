use crate::{command::Command, help, hooks::use_flag, Argv, CliMeta};

pub fn global_flags(argv: &Argv, registered_commands: &[Command], meta: CliMeta) -> bool {
  let is_version = use_flag("version", Some('v'), &argv.flags).try_into().unwrap_or(false);
  let is_help = use_flag("help", Some('h'), &argv.flags).try_into().unwrap_or(false);

  match (is_version, is_help) {
    (true, _) => {
      println!("v{}", meta.version);
      is_version
    }
    (_, true) => {
      help::help(registered_commands, &argv.commands, meta);
      is_help
    }
    (_, _) => false,
  }
}
