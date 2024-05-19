#![deny(warnings, missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

// region: Imports

/// Structs for easliy building entities describing the commands and flags.
pub mod builder;
/// Error enum
pub mod error;
mod runtime;
pub use runtime::*;

/// Run
pub mod run;

mod prelude;
pub use clier_parser::Argv;

// use run::Meta;
use std::env::args;

// endregion: Imports

// region: States
/// Typestate pattern: State for (CliMeta)
#[derive(Debug, Default, Clone)]
pub struct MissingMeta;
/// Typestate pattern: State for (CliMeta)
#[derive(Debug, Clone)]
pub struct HasMeta(pub CliMeta);
/// .
#[derive(Debug, Clone)]
pub struct NotRunnable;
/// .
#[derive(Debug, Clone)]
pub struct Runnable(pub Box<[Commands]>);
// endregion: States

/// Clier is the main struct for the framework
#[derive(Clone, Debug, Default)]
pub struct Clier<T, R> {
  pub(crate) cli_meta: T,
  // pub(crate) options: T,
  pub(crate) registered_commands: R,
  /// Parsed arguments from the command line
  pub(crate) args: Argv,
}

/// Should be used when describing your cli app, this will show up in your help message
#[derive(Clone, Debug, Default)]
pub struct CliMeta {
  /// The name or executable name of your app. Preferably this should be your executable
  /// for the reason of clarity.
  pub name: String,
  /// The description of your app, for example this could include what the usecase for this
  /// executable can be
  pub description: String,
  /// .
  pub version: Option<String>,
  /// .
  pub usage: Option<String>,
}

impl CliMeta {
  pub fn new(name: String, description: String) -> Self {
    Self { name, description, version: None, usage: None }
  }

  pub fn version(mut self, version: impl Into<String>) -> Self {
    self.version = Some(version.into());
    self
  }

  pub fn usage(mut self, usage: impl Into<String>) -> Self {
    self.version = Some(usage.into());
    self
  }
}

/// testing
#[derive(Debug, Clone)]
pub struct CmdMeta {
  /// The name
  pub name: String,
  /// The description
  pub description: String,
}
/// meta
impl CmdMeta {
  /// TODO
  pub fn new(name: &str, description: &str) -> Self {
    CmdMeta { name: name.into(), description: description.into() }
  }
}

/// .
#[derive(Debug, Clone)]
pub struct CmdCollection {
  /// .
  pub meta: CmdMeta,
  /// .
  pub children: Box<[Commands]>,
}

/// Commands
#[derive(Debug, Clone)]
pub enum Commands {
  /// Sigle command
  Command {
    /// TODO
    meta: CmdMeta,
    /// TODO
    handler: fn(clier: Clier<HasMeta, Runnable>) -> crate::run::ExitCode,
  },
  /// collection of commands
  Collection(CmdCollection),
}

impl Clier<HasMeta, NotRunnable> {
  /// TODO
  pub fn runnable(self, cmds: Vec<Commands>) -> Clier<HasMeta, Runnable> {
    Clier { args: self.args, cli_meta: self.cli_meta, registered_commands: Runnable(cmds.into()) }
    // Self {
    //   registered_commands:
    // }
    // self.registered_commands = Runnable(cmds);
    // return self;
  }
}

impl Clier<MissingMeta, NotRunnable> {
  /// .
  pub fn meta(self, meta: CliMeta) -> Clier<HasMeta, NotRunnable> {
    Clier {
      args: self.args,
      cli_meta: HasMeta(meta),
      registered_commands: self.registered_commands,
    }
  }
  /// Create a new [Clier] instance and parsing
  pub fn parse() -> Clier<MissingMeta, NotRunnable> {
    Clier {
      registered_commands: NotRunnable,
      args: Argv::from(&args().collect::<Vec<String>>()[1..]),
      cli_meta: MissingMeta,
    }
  }
  /// Creating a new [Clier] instance with custom arguments
  pub fn with_args(args: &[String]) -> Clier<MissingMeta, NotRunnable> {
    Clier { registered_commands: NotRunnable, args: Argv::from(args), cli_meta: MissingMeta }
  }
}

/// Short for generating command with [Command::new](crate::builder::RCommand)
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
#[macro_export]
macro_rules! cmd {
  ($cmd_name:expr, $desc:expr, $function:expr) => {
    $crate::builder::RCommand::new($cmd_name, $desc, $function)
  };
}
