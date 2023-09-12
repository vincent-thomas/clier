pub mod command;

use std::{env::current_dir, path::PathBuf};

pub trait Generator {
    fn generate(self) -> Result<(), ()>;

    fn find_path() -> Result<PathBuf, ()> {
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
    }
}
