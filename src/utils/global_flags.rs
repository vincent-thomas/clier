use crate::{command::Command, help, hooks::use_flag, CliMeta};

pub fn global_flags(
    flags: &[(String, String)],
    commands: &[String],
    registered_commands: &[Command],
    meta: CliMeta,
) -> bool {
    let is_version = use_flag("version", Some('v'), flags).try_into().unwrap_or(false);
    let is_help = use_flag("help", Some('h'), flags).try_into().unwrap_or(false);

    match (is_version, is_help) {
        (true, _) => {
            println!("v{}", meta.version);
            is_version
        }
        (_, true) => {
            help::help(registered_commands, commands, meta);
            is_help
        }
        (_, _) => false,
    }
}
