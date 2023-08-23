use crate::{Command, App};

pub fn use_help(commands: Vec<Command>, options: App) {
    let longest_c_name = commands
        .iter()
        .map(|value| value.name.len())
        .max()
        .expect("No commands found in vec");
    let prog_name = if std::env::consts::OS == "windows" {
        format!("{}.exe", options.name)
    } else {
        options.name.to_string()
    };

    let mut help_text: Vec<String> = vec![format!(
        "{} v{}\n{}\n",
        prog_name, options.version, options.description
    )];
    if options.usage.is_some() {
        help_text.push(format!("Usage: {} {}\n", prog_name, options.usage.unwrap()));
    }
    help_text.push("Commands:\n".to_string());
    commands.iter().for_each(|command| {
        help_text.push(format!(
            "{:width$} - {}{usage}{note}",
            command.name,
            command.description,
            width = longest_c_name,
            usage = if command.usage.is_some() {
                format!(", Usage: {}", command.usage.unwrap())
            } else {
                "".to_string()
            },
            note = if command.help_string.is_some() {
                format!(", Note: {}", command.help_string.unwrap())
            } else {
                "".to_string()
            }
        ));
    });
    println!("{}\n", help_text.join("\n"));
    std::process::exit(0);
}


