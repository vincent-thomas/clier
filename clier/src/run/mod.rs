pub(crate) mod help;
use crate::display::{label::LabelLogger, Displayer};
use crate::hooks::{use_flag, FlagError};
use crate::run::help::help_renderer;
use crate::{Clier, CmdCollection, Commands, HasMeta, Runnable};
use std::fmt::Debug;
use std::process::Termination;

/// ExitCode is a wrapper around i32 to implement Termination trait
#[derive(Debug, Clone)]
pub struct ExitCode(pub i32);

impl Termination for ExitCode {
  fn report(self) -> std::process::ExitCode {
    std::process::exit(self.0)
  }
}

impl From<i32> for ExitCode {
  fn from(code: i32) -> Self {
    Self(code)
  }
}

impl Clier<HasMeta, Runnable> {
  fn resolve_global_flags(&self) {
    let log = LabelLogger::default();
    let help_flag: Result<bool, FlagError> = use_flag("help", Some('h'), self).try_into();
    let version_flag: Result<bool, FlagError> = use_flag("version", Some('v'), self).try_into();

    match (help_flag, version_flag) {
      (Ok(_), Ok(_)) => {
        log.error("Can't use flags --help and --version at the same time");
        std::process::exit(1);
      }
      (Ok(_), _) => {
        help_renderer(&self.cli_meta.0, self.registered_commands.0.as_ref(), None);
        std::process::exit(0);
      }
      (_, Ok(_)) => {
        let version = self.cli_meta.0.version;

        match version {
          Some((major, minor, patch)) => {
            println!("v{major}.{minor}.{patch}");
          }
          None => log.error("No version was provided"),
        }
        std::process::exit(0);
      }
      (Err(FlagError::Unexisting), Err(FlagError::Unexisting)) => {}
      (_, _) => {
        log.error("Unknown input");
        std::process::exit(0);
      }
    }
  }
  /// .
  pub fn run(self) -> ExitCode {
    self.resolve_global_flags();
    let mut command_to_exec = None;

    let mut commands_to_check = self.registered_commands.0.clone();

    let mut current_parent_collection = None;

    for command in &self.args.commands {
      let mut index: usize = 0;

      loop {
        if commands_to_check.is_empty() || index > commands_to_check.as_ref().len() - 1 {
          break;
        }

        match commands_to_check[index].clone() {
          Commands::Command { meta, handler } => {
            if &meta.name == command {
              command_to_exec = Some(handler);
              break;
            }
          }
          Commands::Collection(collection) => {
            if &collection.meta.name == command {
              commands_to_check = collection.children;
              current_parent_collection =
                Some(CmdCollection { meta: collection.meta, children: commands_to_check.clone() });
              index = 0;
              continue;
            }
          }
        }

        if command_to_exec.is_some() {
          break;
        }

        index += 1;
      }
    }

    let log = LabelLogger::default();
    match command_to_exec {
      None => {
        if let Some(subcommand) = current_parent_collection {
          help_renderer(&self.cli_meta.0, &subcommand.children, Some(subcommand.meta.name));
        } else {
          help_renderer(&self.cli_meta.0, &self.registered_commands.0, None);
        }
        println!();
        log.error("No command found");
        ExitCode(1)
      }
      Some(command) => command(self),
    }
  }
}
