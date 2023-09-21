use crate::builder::Command;
use crate::builder::Flag;
use crate::format::command;
use crate::CliMeta;
use console::{style, Term};

use crate::prelude::*;

fn help_renderer(
  root_command: Option<Vec<Command>>,
  name: String,
  usage: Option<String>,
  version: String,
  description: String,
  flags: Option<Vec<Flag>>,
) {
  let render = Term::stdout();

  render.write_line(&f!("{} v{}", name, version)).unwrap();
  render.write_line(&description).unwrap();

  if let Some(usage) = usage {
    let _ = render.write_line(&f!("\n{}:\n  {name} {usage}", style("Usage").underlined()));
  }

  if let Some(commands) = root_command.clone() {
    if !commands.is_empty() {
      let longest_c_name = commands.iter().map(|value| value.name.len()).max();
      let _ = render.write_line(&style("\nCommands:").underlined().to_string());

      commands.iter().for_each(|command| {
        let _ = render.write_line(&f!(
          "  {:width$}  {}",
          command.name,
          command.description,
          width = longest_c_name.unwrap()
        ));
      })
    }
  }
  if let Some(flags) = flags {
    let _ = render.write_line(&style("\nFlags:").underlined().to_string());
    flags
      .into_iter()
      .for_each(|flag| render.write_line(&f!("  {}: {}", flag.name, flag.description)).unwrap());
  }
  let _ = render.write_line(&style("\nGlobal Flags:").underlined().to_string());
  let _ = render.write_line("  --help, -h     Shows this");
  let _ = render.write_line("  --version, -v  Shows version");
}

pub fn help(commands: &[Command], args: &[String], options: CliMeta) {
  let prog_name = if std::env::consts::OS == "windows" {
    f!("{}[.exe]", options.name)
  } else {
    options.name.to_string()
  };

  let matcher = command::matcher(commands, args);

  if let Some(child_command) = matcher {
    help_renderer(
      child_command.children,
      prog_name,
      child_command.usage.map(|usage| usage.to_string()),
      options.version,
      options.description,
      child_command.flags,
    );
  } else {
    help_renderer(
      Some(commands.to_vec()),
      prog_name,
      options.usage,
      options.version,
      options.description,
      None,
    )
  }
}
