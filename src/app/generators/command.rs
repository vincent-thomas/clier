use super::Generator;
use serde::{Deserialize, Serialize};
use std::io::prelude::*;

pub struct CommandGenerator;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    command_dir: Box<str>,
}

impl Generator for CommandGenerator {
    fn generate() -> Result<(), ()> {
        let path = CommandGenerator::find_path().unwrap();
        let mut config = String::from("");
        let _ = std::fs::File::open(path)
            .unwrap()
            .read_to_string(&mut config)
            .expect("Could not read file");

        let config: Config = serde_json::from_str(&config).unwrap();
        let dir = config.clone().command_dir;

        println!("{dir}");
        Ok(())
    }
}
