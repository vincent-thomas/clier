use std::collections::HashMap;

use crate::builder::{RFlag, RunnableCommand};
use crate::prelude::*;

use crate::Meta;
use console::{style, Term};

fn help_renderer(
  root_command: HashMap<String, &RunnableCommand>,
  name: String,
  usage: Option<String>,
  version: String,
  description: String,
  flags: Vec<RFlag>,
) {
  let render = Term::stdout();

  render.write_line(&f!("{} v{}", name, version)).unwrap();
  render.write_line(&description).unwrap();

  if let Some(usage) = usage {
    let _ = render.write_line(&f!("\n{}:\n  {name} {usage}", style("Usage").underlined()));
  }

  let longest_c_name = root_command.keys().map(|name| name.len()).max();

  if !root_command.is_empty() {
    let _ = render.write_line(&style("\nCommands:").underlined().to_string());
  }

  for (name, command) in root_command {
    let _ = render.write_line(&f!(
      "  {:width$}  {}",
      name,
      command.description,
      width = longest_c_name.unwrap()
    ));
  }

  if !flags.is_empty() {
    let _ = render.write_line(&style("\nFlags:").underlined().to_string());
    flags
      .into_iter()
      .for_each(|flag| render.write_line(&f!("  {}: {}", flag.name, flag.description)).unwrap());
  }
  let _ = render.write_line(&style("\nGlobal Flags:").underlined().to_string());
  let _ = render.write_line("  --help, -h     Shows this");
  let _ = render.write_line("  --version, -v  Shows version");
}

/// Prints help message
pub(crate) fn help(commands: &HashMap<String, RunnableCommand>, args: &[String], options: Meta) {
  let prog_name = if std::env::consts::OS == "windows" {
    f!("{}[.exe]", options.name)
  } else {
    options.name.to_string()
  };

  let matcher = commands.get(args.join(".").as_str());
  let children: Vec<(String, &RunnableCommand)> = if args.is_empty() {
    commands
      .iter()
      .filter(|v| !v.0.contains('.') && v.0 != "root")
      .map(|v| (v.0.clone(), v.1))
      .collect()
  } else {
    commands
      .iter()
      .filter_map(|v| {
        let tes = v.0.starts_with(&args.join(".")) && v.0.clone() != args.join(".");
        if tes {
          Some((v.0.strip_prefix(&f!("{}.", args.join("."))).unwrap().to_string(), v.1))
        } else {
          None
        }
      })
      .collect()
  };
  if commands.get(args.join(".").as_str()).is_none() {
    if args.is_empty() {
      help_renderer(
        HashMap::from_iter(children),
        prog_name,
        options.usage,
        options.version,
        options.description,
        vec![],
      )
    }
    return;
  };
  let flags = if let Some(m) = matcher { m.clone().flags.unwrap_or(vec![]) } else { vec![] };

  help_renderer(
    HashMap::from_iter(children),
    prog_name,
    options.usage,
    options.version,
    options.description,
    flags,
  )

  // let children_commands = match (matcher, is_root) {
  //   (None, false) => commands
  //     .iter()
  //     .filter(|v| v.0.contains('.'))
  //     .map(|v| (v.0.to_owned(), v.1))
  //     .collect::<Vec<(String, &RunnableCommand)>>(),
  //   (None, true) => commands
  //     .iter()
  //     .filter(|v| !v.0.contains('.'))
  //     .map(|v| (v.0.to_owned(), v.1))
  //     .collect::<Vec<(String, &RunnableCommand)>>(),

  //   (Some(_), _) => {
  //     commands.iter().map(|v| (v.0.to_owned(), v.1)).collect::<Vec<(String, &RunnableCommand)>>()
  //   }
  // };

  // let children_map: HashMap<String, &RunnableCommand> = HashMap::from_iter(
  //   children_commands
  //     .iter()
  //     .filter_map(|v| {
  //       println!("{:?} {:?} {args:?}", v.0.split(".").collect::<Vec<&str>>() == args, v.0);
  //       Some((v.0.to_owned(), v.1))
  //       // if let Some(command_name) = v.0.strip_prefix(f!("{}.", &args.join(".").as_str()).as_str()) {
  //       //   Some((command_name.to_owned(), v.1))
  //       // } else {
  //       //   None
  //       // }
  //     })
  //     .collect::<Vec<(String, &RunnableCommand)>>(),
  // );

  // let flags = if let Some(m) = matcher { m.clone().flags.unwrap_or(vec![]) } else { vec![] };

  // help_renderer(
  //   children_map,
  //   prog_name,
  //   options.usage,
  //   options.version,
  //   options.description,
  //   flags,
  // );
  // let matcher = command::matcher(commands, args);

  // if let Some(child_command) = matcher {
  //   help_renderer(
  //     child_command.children,
  //     prog_name,
  //     child_command.usage,
  //     options.version,
  //     options.description,
  //     child_command.flags,
  //   );
  // } else {
  //   help_renderer(
  //     Some(commands),
  //     prog_name,
  //     options.usage,
  //     options.version,
  //     options.description,
  //     None,
  //   )
  // }
}
