use clier::run::Runnable;
use clier::{CliMeta, Clier, ExitCode};

use clier::command::Command;
use clier::hooks::Flag;

fn main() -> ExitCode {
  let meta = CliMeta {
    name: "clier-example-simple".to_string(),
    description: "This is the description".to_string(),
    version: "1.0.0".to_string(),
    usage: None,
  };
  let clier = Clier::parse();

  let exit_code = clier.meta(meta).add_command(test_command()).run();
  exit_code.unwrap()
}

fn test_command() -> Command {
  Command::new("test", "detta är hur man gör", |_args| {
    println!("{:?}", _args);
    0
  })
  .usage("test")
  .flags(vec![Flag::new("tes", "testing".to_string()).short('t')])
}
