// use std::collections::HashMap;

// use crate::builder::{RFlag, RunnableCommand};
// use crate::Meta;
// use crate::{prelude::*, Argv};
// use console::{style, Term};

// use super::resolver::command_fetcher;

// fn help_renderer(
//   root_command: HashMap<String, &RunnableCommand>,
//   name: String,
//   usage: Option<String>,
//   version: String,
//   description: String,
//   flags: Vec<RFlag>,
// ) {
//   let render = Term::stdout();

//   render.write_line(&f!("{} v{}", name, version)).unwrap();
//   render.write_line(&description).unwrap();

//   if let Some(usage) = usage {
//     let _ = render.write_line(&f!("\n{}:\n  {name} {usage}", style("Usage").underlined()));
//   }

//   let longest_c_name = root_command.keys().map(|name| name.len()).max();

//   if !root_command.is_empty() {
//     let _ = render.write_line(&style("\nCommands:").underlined().to_string());
//   }

//   for (name, command) in root_command {
//     let _ = render.write_line(&f!(
//       "  {:width$}  {}",
//       name,
//       command.description,
//       width = longest_c_name.unwrap()
//     ));
//   }

//   if !flags.is_empty() {
//     let _ = render.write_line(&style("\nFlags:").underlined().to_string());
//     flags
//       .into_iter()
//       .for_each(|flag| render.write_line(&f!("  {}: {}", flag.name, flag.description)).unwrap());
//   }
//   let _ = render.write_line(&style("\nGlobal Flags:").underlined().to_string());
//   let _ = render.write_line("  --help, -h     Shows this");
//   let _ = render.write_line("  --version, -v  Shows version");
// }

// /// Prints help message
// pub(crate) fn help(commands: &HashMap<String, RunnableCommand>, args: &[String], options: Meta) {
//   let prog_name = if std::env::consts::OS == "windows" {
//     f!("{}[.exe]", options.name)
//   } else {
//     options.name.to_string()
//   };

//   // let matcher = commands.get(args.join(".").as_str());
//   let children: Vec<(String, &RunnableCommand)> = if args.is_empty() {
//     commands
//       .iter()
//       .filter(|v| !v.0.contains('.') && v.0 != "root")
//       .map(|v| (v.0.clone(), v.1))
//       .collect()
//   } else {
//     commands
//       .iter()
//       .filter_map(|v| {
//         let starts_with_valid_path = v.0.starts_with(&args.join("."));
//         let is_actual_path = v.0.clone() != args.join(".");
//         if starts_with_valid_path && is_actual_path {
//           Some((v.0.strip_prefix(&f!("{}.", args.join("."))).unwrap().to_string(), v.1))
//         } else {
//           None
//         }
//       })
//       .collect()
//   };

//   let (args, main_command) = command_fetcher(&Argv::from(args), commands.clone());

//   if main_command.is_none() {
//     if args.is_empty() {
//       help_renderer(
//         HashMap::from_iter(children),
//         prog_name,
//         options.usage,
//         options.version,
//         options.description,
//         vec![],
//       )
//     }
//     return;
//   };
//   let flags =
//     if let Some(ref m) = main_command { m.clone().flags.unwrap_or(vec![]) } else { vec![] };

//   help_renderer(
//     HashMap::from_iter(children),
//     prog_name,
//     options.usage,
//     // main_command.unwrap().usage,
//     options.version,
//     options.description,
//     flags,
//   )
// }
