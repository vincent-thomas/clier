use std::fs::{create_dir, File};
use std::io::Write;
use std::path::Path;

pub struct ProjectGenerator;

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

impl ProjectGenerator {
  pub fn generate(name: impl Into<String>, description: impl Into<String>) {
    let name = name.into();
    let description = description.into();
    let path = Path::new(&name);

    if !path.exists() {
      let _ = create_dir(path);
    }

    let _ = create_dir(path.join("src"));

    let _ = File::create(path.join("Cargo.toml")).unwrap().write_all(
      format!(
        "
[package]
name = \"{name}\"
version = \"0.0.1\"
edition = \"2021\"
description = \"{description}\"

[dependencies]
clier = \"{PKG_VERSION}\"
",
      )
      .as_bytes(),
    );

    let _ = File::create(path.join("src/main.rs"))
      .unwrap()
      .write_all(include_bytes!("../../../examples/parser.rs"));
  }
}
