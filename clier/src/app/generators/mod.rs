use std::{
  fs::{read_dir, DirEntry, File},
  io::BufReader,
  path::Path,
};

use serde::{Deserialize, Serialize};
mod command;
pub use command::*;
mod project;
pub use project::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
  pub command_dir: String,
}

pub fn get_config() -> Config {
  let dir = read_dir(".").unwrap();
  let mut config_file: Option<DirEntry> = None;

  for i in dir {
    if config_file.is_some() {
      break;
    }
    let i = i.unwrap();
    if i.file_name() == "clier.config.json" {
      config_file = Some(i);
    }
  }

  let Some(file_config) = config_file else {
      eprintln!("clier.config.json could not be found");
      std::process::exit(1);
    };

  let file = File::open(Path::new(&file_config.path())).unwrap();
  let reader = BufReader::new(file);

  serde_json::from_reader(reader).unwrap()
}
