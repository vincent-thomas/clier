//! # Command Line Argument Parser for Rust
//! `Clier` is a command line argument parser and command framework for rust.
//!
//! ## Parser
//! To start a new cli projects run:
//!
//! ```console
//! $ cargo new demo
//! $ cargo add clier
//! ```
//!
//! Then define your CLI in `main.rs`:
//!
//! ```rust

//! use clier::Argv;
//! use clier::Clier;
//!
//! fn main() {
//!   let args: Argv = Clier::parse().args;
//!   println!("{:#?}", args);
//! }
//!
//! ```
//!
//! And try it out:
//! ```md
//!
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
//! soon...

pub mod builder;
pub mod error;
pub mod help;
pub mod hooks;
pub mod run;

mod format;
mod parser;
mod prelude;
pub use parser::Argv;

use std::env::args;
use std::fmt::{Debug, Formatter};
use std::process::Termination;

#[derive(Debug, Clone, Default)]
pub struct CliMeta {
  pub name: String,
  pub description: String,
  pub usage: Option<String>,
  pub version: String,
}

// region: Meta States
#[derive(Debug, Default, Clone)]
pub struct MissingMeta;
#[derive(Debug, Default, Clone)]
pub struct AlreadyHasMeta(pub(crate) CliMeta);
// endregion: Meta States

#[derive(Clone, Default)]
pub struct Clier<T> {
  pub(crate) options: T,
  pub(crate) registered_commands: Vec<builder::Command>,
  pub args: Argv,
}

#[derive(Debug, Clone)]
pub struct ExitCode(pub i32);

impl Termination for ExitCode {
  fn report(self) -> std::process::ExitCode {
    std::process::exit(self.0)
  }
}

impl<T: Debug> Debug for Clier<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Clier").field("args", &self.args).finish()
  }
}

impl Clier<MissingMeta> {
  pub fn parse() -> Clier<MissingMeta> {
    Clier {
      options: MissingMeta,
      registered_commands: vec![],
      args: parser::transform(&args().collect::<Vec<String>>()[1..]).unwrap(),
    }
  }

  pub fn parse_with_vargs(args: &[String]) -> Clier<MissingMeta> {
    Clier {
      options: MissingMeta,
      registered_commands: vec![],
      args: parser::transform(args).unwrap(),
    }
  }
}
