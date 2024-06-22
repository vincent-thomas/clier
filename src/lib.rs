#![deny(warnings, missing_docs)]
#![allow(dead_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! ** Rust command Line app framework **
//!
//! ## Example
//! Run
//! ```sh
//! cargo add clier
//!
//! ```
//!
//! Then create a file `src/main.rs` with the following content:
//! ```rust
//! use clier::{clier, Parser, command, ExitCode, Argv};
//! use clier::prelude::*;
//! fn main() -> ExitCode {
//!   let mut app = clier!();
//!   app.register(build::new(&app));
//!   app.register(test::new(&app));
//!   return app.run();
//! }
//!
//! #[derive(Parser, Clone, Debug)]
//! struct Flags {
//!   /// The description
//!   #[meta(short = 'n')]
//!   name: Option<String>,
//!   test: bool,
//! }
//! /// This would be the command description in the help message
//! #[command]
//! fn test(argv: Argv) -> clier::ExitCode {
//!   println!("args {:?}", argv);
//!   ExitCode(0)
//! }
//! /// Another command
//! #[flags = Flags)]
//! fn build(argv: Argv, flags: Flags) -> clier::ExitCode {
//!   println!("args {:?}", argv);
//!   println!("flags {:?}", flags.name);
//!   println!("flags {:?}", flags.test);
//!   ExitCode(0)
//! }
//! ```
//! And use it:
//! ```sh
//! $ cargo run -- -h
//! your_app_name@0.1.0
//! Your simple description from Cargo.toml
//!
//! COMMANDS:
//!  test - This would be the command description in the help message
//!  build - Another command
//! ```
//!
mod display;
pub use clier_derive::*;
pub use display::*;
use hooks::FlagError;
/// hooks and stuff
pub mod hooks;
use std::collections::HashMap;

use clier_utils::MetaValue;
mod exitcode;
pub use exitcode::*;

/// Mostly imports structs and traits used under the hood. Checkout the source code for what it
/// brings to scope.
pub mod prelude;
pub use clier_parser::Argv;

/// .
pub trait FlagParser {
  /// .
  #[allow(clippy::result_unit_err)]
  fn parse() -> (Self, HashMap<String, FlagError>)
  where
    Self: Sized;
}

/// .
pub trait Command {
  /// .
  fn name(&self) -> &'static str;
  /// .
  fn description(&self) -> &'static str;
  /// .
  fn execute(&self, clierv2: &Clier) -> ExitCode;
  /// .
  fn flags(&self, clierv2: &Clier) -> Vec<MetaValue>;
}

/// .
pub struct Clier {
  name: String,
  description: String,
  version: Option<String>,
  /// command line arguments from [clier::Argv]
  pub argv: clier_parser::Argv,
  commands: HashMap<String, Box<dyn Command>>,
}

impl Clier {
  ///.
  pub fn new(name: impl Into<String>, description: impl Into<String>, args: Argv) -> Self {
    Self {
      name: name.into(),
      description: description.into(),
      version: None,
      argv: args,
      commands: HashMap::new(),
    }
  }
  /// .
  pub fn version(self, version: impl Into<String>) -> Self {
    Self { version: Some(version.into()), ..self }
  }
  /// .
  pub fn register(&mut self, func: impl Command + 'static) {
    self.commands.insert(func.name().to_string(), Box::new(func));
  }

  /// .
  pub fn help(&self) -> ExitCode {
    let mut header = self.name.clone();

    if self.version.is_some() {
      header.push_str(&format!("@{}", self.version.clone().unwrap()));
    }
    header.push_str(&format!("\n{}", self.description));

    let commands_without_root: HashMap<String, &Box<dyn Command>> =
      self.commands.iter().filter(|(k, _)| k != &"root").map(|(k, v)| (k.clone(), v)).collect();

    println!("{header}\n");

    if self.commands.contains_key("root") {
      println!("Command '{}':", self.name);
      let command = self.commands.get("root").unwrap();

      for flag in command.flags(self) {
        let mut help = format!("    --{}", flag.long);
        if flag.short.is_some() {
          help.push_str(&format!(", -{}", flag.short.unwrap()));
        }
        if !flag.description.is_empty() {
          help.push_str(&format!("\n        {}", flag.description.replace('\n', "\n        ")));
        }
        help.push_str(&format!(" {}", if flag.optional { "(required)" } else { "" }));
        println!("{}\n", help);
      }
    }

    println!("COMMANDS:");
    for (name, item) in commands_without_root {
      let mut command_help = format!("    {}", console::style(name).underlined());

      if !item.description().is_empty() {
        command_help.push_str(&format!("\n{}", item.description()).replace('\n', "\n        "));
      }

      println!("{}", command_help);
      let flags = item.flags(self);

      if flags.is_empty() {
        continue;
      }
      println!("\n      Flags:");

      for flag in flags {
        let mut help = format!("      {}", console::style(format!("--{}", flag.long)).dim());
        if flag.short.is_some() {
          help
            .push_str(&format!(", {}", console::style(format!("-{}", flag.short.unwrap())).dim()));
        }
        if !flag.optional {
          let required_thing = console::style("required").red();
          help.push_str(&format!("   ({})", required_thing));
        }
        if !flag.description.is_empty() {
          help.push_str(&format!("\n        {}", flag.description.replace('\n', "\n        ")));
        }
        println!("{}\n", help);
      }
    }

    println!("GLOBAL FLAGS:");
    println!("    --help, -h: Show this message");
    println!("    --version, -v: Show version");

    ExitCode(0)
  }
  /// .
  pub fn run(&self) -> ExitCode {
    let help_flag = self.argv.flags.get("help").or(self.argv.flags.get("h"));

    if help_flag.is_some_and(|value| &*value.clone() == "true") {
      return self.help();
    }

    if self.argv.commands.is_empty() {
      match self.commands.get("root") {
        None => {
          return self.help();
        }
        Some(command) => {
          return command.execute(&self);
        }
      }
    }

    let to_match_against = self.argv.commands.join(" ");

    match self.commands.get(&to_match_against) {
      Some(command) => command.execute(self),
      None => self.help(),
    }
  }
}
