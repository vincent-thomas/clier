use serde::{Deserialize, Serialize};
use std::{
  fs::{read_dir, DirEntry, File},
  io::BufReader,
  path::Path,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
  pub command_dir: String,
}
impl Config {
  pub fn get() -> Config {
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
}
