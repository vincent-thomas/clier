use crate::command::Command;
use crate::hooks::Flag;
use crate::utils::match_command;
use crate::CliMeta;

fn help_renderer(
  root_command: Option<Vec<Command>>,
  name: String,
  usage: Option<String>,
  version: String,
  description: String,
  flags: Option<Vec<Flag>>,
) {
  let mut help_text = vec![format!("{} v{}\n{}", name, version, description)];

  if let Some(usage) = usage {
    help_text.push(format!("\nUsage:\n  {} {}", name, usage));
  }

  if let Some(commands) = root_command.clone() {
    if !commands.is_empty() {
      let longest_c_name = commands.iter().map(|value| value.name.len()).max();
      help_text.push("\nCommands:".to_string());

      commands.iter().for_each(|command| {
        help_text.push(format!(
          "  {:width$}  {}",
          command.name,
          command.description,
          width = longest_c_name.unwrap()
        ));
      })
    }
  }
  if let Some(flags) = flags {
    help_text.push("\nFlags:".to_string());
    flags
      .into_iter()
      .for_each(|flag| help_text.push(format!("  {}: {}", flag.name, flag.description)));
  }
  help_text.push("\nGlobal Flags:".to_string());
  help_text.push("  --help, -h     Shows this".to_string());
  help_text.push("  --version, -v  Shows version".to_string());
  println!("{}", help_text.join("\n"));
}

pub fn help(commands: &[Command], args: &[String], options: CliMeta) {
  let prog_name = if std::env::consts::OS == "windows" {
    format!("{}[.exe]", options.name)
  } else {
    options.name.to_string()
  };

  let matcher = match_command(commands, args);

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
