use crate::parser::Argv;

mod flags;

pub use flags::*;

/// The CmdArgs struct that is passed to all command handlers.
#[derive(Debug, Clone)]
pub struct CmdArgs {
  /// struct 'Argv' contains parsed flags and commands.
  pub args: Argv,
  /// Registered flags for the command by the struct 'Command::flag'.
  pub registered_flags: Vec<(String, flags::Flag)>,
}

pub type Handler = fn(args: CmdArgs) -> i32;

/// The Command struct to initialize a new command.
/// ## Non-complete example:
/// ```rust
/// use clier::builder::{Command, Flag};
///
/// let command = Command::new(
/// /* command name: */ "command",
/// /* description: */ "description",
/// /* handler: */ |_args| {
///   /* Your logic */
///   0 // <-- i32: Exit Code of program, success = 0
/// })
/// .usage("command [usage text]")
/// .flag("flag-name", Some('f'), "flag description" /* <-- In help */);
/// ```
/// Alot of these properties/builder methods are no necesserialy required, but are usefull for the user in the help output.
/// ## Subcommand
/// It is also possible to add subcommands to a command:
/// ```rust
/// use clier::builder::{Flag, Command};
///
/// let command = Command::new(
/// /* command name: */ "command",
/// /* description: */ "description",
/// /* handler: */ |_args| {
///   /* Your logic */
///   0 // <-- i32: Exit Code of program, success = 0
/// })
/// .usage("command [usage text]")
/// .flag("flag-name", Some('t'), "flag description" /* <-- In help */);
///
/// command.subcommand(
///   "subcommand",
///   "description",
///   Some("usage"),
///   |_args| {
///    /* Your logic */
///    0 // <-- i32: Exit Code of program, success = 0
/// });
/// ```
///
/// It has almost the same methods and builder methods as a [Command]

#[derive(Debug, Clone)]
pub struct Command {
  pub name: String,
  pub handler: Handler,
  pub usage: Option<String>,
  pub flags: Option<Vec<flags::Flag>>,
  pub description: String,
  pub children: Option<Vec<Command>>,
}

impl Command {
  pub fn new(name: &str, description: &str, handler: Handler) -> Self {
    Self {
      name: name.to_string(),
      description: description.to_string(),
      flags: None,
      usage: None,
      handler,
      children: None,
    }
  }

  pub fn usage(mut self, usage: &str) -> Self {
    self.usage = Some(usage.to_string());
    self
  }

  pub fn flag(mut self, name: &str, short: Option<char>, description: &str) -> Self {
    let mut flags = self.flags.unwrap_or(vec![]);

    let mut flag = Flag::new(name, description.to_string());

    if let Some(short) = short {
      flag = flag.short(short);
    }

    flags.push(flag);

    self.flags = Some(flags);
    self
  }

  pub fn subcommand(
    mut self,
    name: &str,
    description: &str,
    usage: Option<&str>,
    handler: Handler,
  ) -> Self {
    let mut new_command = Self::new(name, description, handler);

    if let Some(usage) = usage {
      new_command = new_command.usage(usage);
    }

    self.children.as_mut().unwrap_or(&mut vec![]).push(new_command);
    self
  }
}
