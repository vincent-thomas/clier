use std::path::Path;

pub fn remove_ex(args: &mut Vec<String>) -> Vec<String> {
    let maybe_file = args.get(0).unwrap();
    let path = if std::env::consts::OS == "windows" && maybe_file.ends_with(".exe") {
        format!("{maybe_file}.exe")
    } else {
        args.get(0).unwrap().clone()
    };
    let file = Path::new(&path); //args.get(0).unwrap());
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

pub fn get_value_flag(flag: String) -> (String, String) {
    let is_splittable = flag.contains("=");

    if !is_splittable {
        let is_false = flag.starts_with("no-");
        let key = flag.replace("no-", "");

        return match is_false {
            true => (key, "false".to_string()),
            false => (key, "true".to_string()),
        };
    }

    let flag = flag.split("=").collect::<Vec<&str>>();

    let flag_key = flag.get(0).unwrap().to_string();
    let flag_value = flag.get(1).unwrap().to_string();

    return (flag_key, flag_value);
}

#[derive(Debug)]
pub struct Args {
    pub commands: Vec<String>,
    pub flags: Vec<(String, String)>,
}

pub(crate) fn format_args(args_with_ex: &Vec<String>) -> Args {
    let args = remove_ex(&mut args_with_ex.to_owned());

    let mut commands = vec![];
    let mut flags = vec![];

    args.iter().for_each(|value| {
        if value.starts_with("--") {
            flags.push(value.clone());
        } else {
            commands.push(value.clone())
        }
    });

    let flags: Vec<(String, String)> = flags
        .iter()
        .map(|v| v.replace("--", ""))
        .map(get_value_flag)
        .collect();

    return Args { commands, flags };
}
