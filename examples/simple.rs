use clier::run::Runnable;
use clier::{CliMeta, Clier, ExitCode};

use clier::builder::Command;
use clier::hooks::use_flags;

fn main() -> ExitCode {
  let meta = CliMeta {
    name: "clier-example-simple".to_string(),
    description: "This is the description".to_string(),
    version: "1.0.0".to_string(),
    usage: Some("test".to_string()),
  };
  let clier = Clier::parse();

  let exit_code = clier.meta(&meta).command(test_command()).run();
  exit_code.unwrap()
}

fn test_command() -> Command {
  Command::new("test", "detta är hur man gör", |_args| {
    let flags = use_flags(&_args);
    println!("{:?}", flags);
    0
  })
  .usage("test")
  .flag("tes", None, "testing")
}
