#![deny(warnings, missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Command Line Argument Parser for Rust
//! `Clier` is a command line argument parser and command framework for rust.
//!
//! ## Parser
//! To start a new cli projects run:
//!
//! ```console
//! $ cargo new demo && cd demo
//! $ cargo add clier
//! ```
//!
//! Then define your CLI in `src/main.rs`:
//!
//! ```rust
#![doc = include_str!("../examples/parser.rs")]
//! ```
//!
//! And try it out:
//! ```md
//! $ cargo run -- command subcommand --test=value --no-production --help --try-me=false
//! Argv {
//!     commands: [
//!         "command",
//!         "subcommand",
//!     ],
//!     flags: {
//!         "test": "value",
//!         "production": "false",
//!         "help": "true",
//!         "try-me": "false",
//!     },
//! }
//! ```
//!
//! ## Framework
//! To start a new cli app run:
//!
//! ```console
//! $ cargo new demo-app && cd demo-app
//! $ cargo add clier
//! ```
//!
//! Then define your CLI in `src/main.rs`:
//! ```rust
#![doc = include_str!("../examples/framework.rs")]
//! ```

// region: Imports

/// Structs for easliy building entities describing the commands and flags.
pub mod builder;
/// Error enum
pub mod error;
mod runtime;
pub use runtime::*;
/// Short hand for building commands and flags
/// ## Check source for code
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub mod macros;
/// Run
pub mod run;

mod parser;
mod prelude;
pub use parser::Argv;

use run::Meta;
use std::env::args;

// endregion: Imports

// region: Meta States
/// Typestate pattern: State for (CliMeta)
#[derive(Debug, Default, Clone)]
pub struct MissingMeta;
/// Typestate pattern: State for (CliMeta)
#[derive(Debug, Clone)]
pub struct AlreadyHasMeta(pub(crate) Meta);
// endregion: Meta States

/// Clier is the main struct for the framework
#[derive(Clone, Default, Debug)]
pub struct Clier<T> {
  pub(crate) options: T,
  pub(crate) registered_commands: Vec<builder::RCommand>,
  /// Parsed arguments from the command line
  pub args: Argv,
}

impl Clier<MissingMeta> {
  /// Create a new [Clier] instance and parsing
  pub fn parse() -> Clier<MissingMeta> {
    Clier {
      options: MissingMeta,
      registered_commands: vec![],
      args: Argv::from(&args().collect::<Vec<String>>()[1..]),
    }
  }

  /// Creating a new [Clier] instance with custom arguments
  pub fn with_args(args: &[String]) -> Clier<MissingMeta> {
    Clier { options: MissingMeta, registered_commands: vec![], args: Argv::from(args) }
  }
}
