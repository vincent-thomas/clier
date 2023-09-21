use clier_parser::Argv;

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
/// .flags(vec![
///   Flag::new("flag-name", "flag description".to_string() /* <-- In help */)
///   .short('t')
/// ]);
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
/// .flags(vec![
///   Flag::new("flag-name", "flag description".to_string() /* <-- In help */)
///   .short('t')
/// ]);
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

  pub fn flags(mut self, flags: Vec<flags::Flag>) -> Self {
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
