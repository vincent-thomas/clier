use clier::{hooks::use_flag, Command};

use crate::app::generators::{command::CommandGenerator, Generator};

pub fn generate_command() -> Command {
    Command::new(
        "generate",
        "Generates parts of program",
        Some("test"),
        command,
    )
}

fn command(args: clier::Args) -> i32 {
    let test = use_flag("test", Some('t'), &args.flags)
        .try_into()
        .unwrap_or_else(|_| "".to_string());
    println!("Test: {:?}", test);
    let path = {
        let path = current_dir().unwrap();
        let mut item: Option<PathBuf> = None;
        path.ancestors().for_each(|v| {
            for dir in v.read_dir().into_iter() {
                for file in dir.into_iter() {
                    if let Ok(config_file) = file {
                        if config_file.file_name() == "clier.config.json" {
                            item = Some(config_file.path());
                        }
                    }
                }
            }
        });

        // for i in item.clone().into_iter() {
        let mut right = false;

        for i in item
            .clone()
            .into_iter()
            .next()
            .unwrap()
            .parent()
            .unwrap()
            .read_dir()
            .unwrap()
        {
            if let Ok(cargo_toml) = i {
                if cargo_toml.file_name() == "Cargo.toml" {
                    right = true;
                }
            }
        }

        if right {
            item.ok_or(())
        } else {
            Err(())
        }
    };
    println!("{:?}", path);
    0
}
