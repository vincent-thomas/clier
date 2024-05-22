use crate::display::label::LabelLogger;
use crate::display::Displayer;
use crate::prelude::*;
use crate::{CliMeta, CmdMeta, Commands};
use console::{style, Term};

pub(crate) fn help_renderer(
  meta: &CliMeta,
  commands: &[Commands],
  not_root_command_name: Option<String>,
) {
  let term = Term::stdout();
  meta_renderer(&term, meta, not_root_command_name);

  let mut command_groups: Vec<CmdMeta> = Vec::new();
  let mut command_commands: Vec<CmdMeta> = Vec::new();

  for command in commands.iter() {
    match command.clone() {
      Commands::Collection(collection) => {
        command_groups.push(collection.meta);
      }
      Commands::Command { meta, handler: _ } => {
        command_commands.push(meta);
      }
    }
  }

  if !command_groups.is_empty() {
    let _ = term.write_line(f!("\n{}", style("Command Groups:").underlined()).as_str());
  }
  for group in command_groups {
    let _ = term.write_line(f!("  {} - {}", group.name, group.description).as_str());
  }

  if !command_commands.is_empty() {
    let _ = term.write_line(f!("\n{}", style("Commands:").underlined()).as_str());
  }
  for command in command_commands {
    let _ = term.write_line(f!("  {} - {}", command.name, command.description).as_str());
  }

  let _ = term.write_line(f!("\n{}", style("Global Flags:").underlined()).as_str());
  if meta.version.is_some() {
    let _ = term.write_line("  --version - Writes the current version of the program");
  }
  let _ = term.write_line("  --help - Writes this message");
}

fn meta_renderer(term: &Term, meta: &CliMeta, not_root: Option<String>) {
  let version = match &meta.version {
    Some(version) => {
      format!("@{version}")
    }
    None => "".to_string(),
  };
  let _ = term.write_line(f!("{name}{version}", name = meta.name).as_str());
  let _ = term.write_line(&meta.description);
  let mut name = String::from(&meta.name);
  if let Some(root) = not_root {
    name.push_str(f!(" {}", root).as_str());

    let log = LabelLogger::default();

    log.info(format!("Showing help message for command group: {root}").as_str())

    /*     let _ = term.write_line(
      f!("\n{} showing help command for command group: {root}", style("NOTE:").underlined())
        .as_str(),
    ); */
  }
  if let Some(usage) = &meta.usage {
    let _ = term.write_line(f!("\n{}", style("Usage:").underlined()).as_str());
    let _ = term.write_line(f!("  {} {}", name, &usage).as_ref());
  }
}
