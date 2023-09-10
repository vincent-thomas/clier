use crate::{Args, Command};
use std::{collections::HashMap, path::Path};

fn remove_ex(args: &mut Vec<String>) -> Vec<String> {
    let maybe_file = args.get(0).unwrap();
    let path = if std::env::consts::OS == "windows" && maybe_file.ends_with(".exe") {
        format!("{maybe_file}.exe")
    } else {
        args.get(0).unwrap().clone()
    };
    let file = Path::new(&path);
    let maybe_ex = if std::env::consts::OS == "windows" {
        file.extension().unwrap() == "exe"
    } else {
        Path::new(file).is_file()
    };
    if maybe_ex {
        args.remove(0);
    }
    args.to_owned()
}

fn get_value_flag(flag: String) -> (String, String) {
    let is_splittable: bool = flag.contains('=');

    if !is_splittable {
        let is_false = flag.starts_with("no-");
        let key = flag.replace("no-", "");

        return match is_false {
            true => (key, "false".to_string()),
            false => (key, "true".to_string()),
        };
    }

    let flag = flag.split('=').collect::<Vec<&str>>();

    let flag_key = flag.first().unwrap().to_string();
    let flag_value = flag.get(1).unwrap().to_string();

    (flag_key, flag_value)
}

fn parse_flags(raw_flags: &[String]) -> Vec<(String, String)> {
    raw_flags
        .iter()
        .filter(|flag| {
            let flag: Vec<&str> = flag.split('=').collect();
            let key = flag.first().unwrap();

            key.starts_with('-') && key[1..].len() == 1
                || key.starts_with("--") && key[2..].len() != 1
        })
        .map(|v| -> String {
            let flag: Vec<&str> = v.split('=').collect();
            let key = *flag.first().unwrap();
            let is_short = key.starts_with('-') && key[1..].len() == 1;
            if is_short {
                v[1..].to_string()
            } else {
                v[2..].to_string()
            }
        })
        .map(get_value_flag)
        .collect()
}

pub(crate) fn prepare_vargs(args_with_ex: &[String]) -> Args {
    let args = remove_ex(&mut args_with_ex.to_owned());

    let mut commands_to_parse = vec![];
    let mut only_flags_raw: Vec<String> = vec![];

    args.iter().for_each(|value| {
        if value.starts_with('-') || value.starts_with("--") {
            only_flags_raw.push(value.clone());
        } else {
            commands_to_parse.push(value.clone())
        }
    });

    let parsed_flags = parse_flags(&only_flags_raw);

    Args {
        commands: commands_to_parse,
        flags: parsed_flags,
    }
}

pub(crate) fn match_command(
    registered_commands: &[Command],
    commands: &[String],
) -> Option<Command> {
    let mut command_matcher = HashMap::new();
    for command in registered_commands.iter() {
        let mut command_name = command.name.clone().to_string();
        command_matcher.insert(command_name.clone(), command.clone());
        let mut command_to_check = command.clone();

        loop {
            if command_to_check.clone().children.is_some()
                && command_to_check.clone().children.unwrap().is_empty()
            {
                panic!("Use None instead of empty vector");
            } else if command_to_check.clone().children.is_some() {
                for child in command_to_check.children.clone().unwrap() {
                    command_name = format!("{}.{}", command_name.clone(), child.name.clone());
                    command_matcher.insert(command_name.clone(), child.clone());
                    command_to_check = child;
                }
            } else {
                break;
            }
        }
    }

    command_matcher.get(&commands.join(".")).cloned()
}
