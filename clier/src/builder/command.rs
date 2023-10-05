use super::{flag::RFlag, Flag};
use clier_parser::Argv;
// use proc_macro::TokenStream;
// use syn::{self, parse_macro_input, Data, Fields, Ident};

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

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RunnableCommand {
  /// The function to run command.
  pub handler: Handler,
  // /// Usage of the command. Displayed in help.
  // pub usage: Option<String>,
  /// Registered Flags that are required for command to run. Passed down with [crate::hooks::use_flags] hook.
  pub flags: Option<Vec<RFlag>>,
  /// The description of the command.
  pub description: String,
  // / Subcommands of the command.
  // pub children: Option<Vec<RCommand>>,
}

/// The Command struct to initialize a new command.
/// ## Non-complete example:
/// ```rust
/// use clier::builder::{RCommand, Flag};
///
/// let command = RCommand::new(
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

#[derive(Debug, Clone)]
pub struct RCommand {
  /// Name
  pub name: String,
  /// The function to run command.
  pub handler: Handler,
  // / Usage of the command. Displayed in help.
  // pub usage: Option<String>,
  /// Registered Flags that are required for command to run. Passed down with [crate::hooks::use_flags] hook.
  pub flags: Option<Vec<RFlag>>,
  /// The description of the command.
  pub description: String,
  /// Subcommands of the command.
  pub children: Option<Vec<RCommand>>,
}

impl RCommand {
  /// The Command struct to initialize a new command.
  /// ## Non-complete example:
  /// ```rust
  /// use clier::builder::{RCommand, Flag};
  ///
  /// let command = RCommand::new(
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
  /// # Panics
  /// Panics if name contains a dot, internal reasons.
  pub fn new(name: &str, description: &str, handler: Handler) -> Self {
    let name_contains_dot = !name.contains('.');
    assert!(name_contains_dot);

    Self {
      name: name.to_string(),
      description: description.to_string(),
      flags: None,
      // usage: None,
      handler,
      children: None,
    }
  }

  // /// Add usage to [RCommand]
  // pub fn usage(mut self, usage: &str) -> Self {
  //   self.usage = Some(usage.to_string());
  //   self
  // }

  /// Adds a flag to [RCommand]
  pub fn flag(mut self, name: &str, short: Option<char>, description: &str) -> Self {
    let mut flags = self.flags.unwrap_or(vec![]);

    let mut flag = RFlag::new(name, description.to_string());

    if let Some(short) = short {
      flag = flag.short(short);
    }

    flags.push(flag);

    self.flags = Some(flags);
    self
  }
  /// It is possible to add subcommands to a command:
  /// ```rust
  /// use clier::builder::{Flag, RCommand};
  ///
  /// let command = RCommand::new(
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
  /// It has almost the same methods and builder methods as a [RCommand]
  pub fn subcommand(
    mut self,
    name: &str,
    description: &str,
    // usage: Option<&str>,
    handler: Handler,
  ) -> Self {
    let new_command = Self::new(name, description, handler);

    // if let Some(usage) = usage {
    //   new_command = new_command.usage(usage);
    // }
    let mut children = self.children.unwrap_or(vec![]);
    children.push(new_command);
    self.children = Some(children);
    self
  }
}
