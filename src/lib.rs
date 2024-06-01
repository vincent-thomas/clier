#![deny(warnings, missing_docs)]
#![allow(dead_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

mod display;
/// Error enum
pub mod error;
pub use clier_derive::*;
pub use display::*;
/// hooks and stuff
pub mod hooks;
use std::collections::HashMap;

use clier_utils::MetaValue;
mod exitcode;
pub use exitcode::*;

/// .
pub mod prelude;
pub use clier_parser::Argv;
/// .
pub trait FlagParser {
  /// .
  #[allow(clippy::result_unit_err)]
  fn parse(clier2: &ClierV2) -> Self;
}

pub use clier_derive::Parser;
/// .
pub trait Command {
  /// .
  fn name(&self) -> &'static str;
  /// .
  fn description(&self) -> Option<&'static str>;
  /// .
  fn execute(&self, clierv2: &ClierV2) -> ExitCode;
  /// .
  fn flags(&self, clierv2: &ClierV2) -> Vec<MetaValue>;
}

/// .
pub struct ClierV2 {
  name: String,
  description: String,
  version: Option<String>,
  /// .
  pub argv: clier_parser::Argv,
  commands: HashMap<String, Box<dyn Command>>,
}
///.
#[macro_export]
macro_rules! clier {
  () => {
    $crate::ClierV2::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_DESCRIPTION"))
      .version(env!("CARGO_PKG_VERSION"))
  };
}

impl ClierV2 {
  ///.
  pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
    let args = clier_parser::Argv::parse();
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
        if flag.description.is_some() {
          help.push_str(&format!("\n      {}", flag.description.unwrap()));
        }
        help.push_str(&format!(" {}", if flag.optional { "(required)" } else { "" }));
        println!("{}\n", help);
      }
    }

    println!("COMMANDS:");
    for (name, item) in commands_without_root {
      let mut command_help = format!("  {}", name);

      if item.description().is_some() {
        command_help.push_str(&format!(" - {}", item.description().unwrap()));
      }

      println!("{}", command_help);

      for flag in item.flags(self) {
        let mut help = format!("    --{}", flag.long);
        if flag.short.is_some() {
          help.push_str(&format!(", -{}", flag.short.unwrap()));
        }
        if flag.optional {
          help.push_str("   (required)");
        }
        if flag.description.is_some() {
          help.push_str(&format!("\n      {}", flag.description.unwrap()));
        }
        println!("{}\n", help);
      }
    }
    ExitCode(0)
  }
  /// .
  pub fn run(self) -> ExitCode {
    let help_flag = self.argv.flags.get("help").or(self.argv.flags.get("h"));
    if help_flag.is_some_and(|value| value == "true") {
      self.help();
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
      Some(command) => return command.execute(&self),
      None => self.help(),
    }
  }
}
