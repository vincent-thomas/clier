use clier::prelude::*;
use clier::{command, Argv, ExitCode, Parser};

#[clier::main]
fn main(app: &mut clier::Clier) -> ExitCode {
  app.register(build::new(&app));
  app.run()
}

#[derive(Parser, Clone, Debug)]
struct Flags {
  name: String,
}

/// detta är en kommentar
#[command(flags = Flags)]
fn build(argv: Argv, flags: Flags) -> clier::ExitCode {
  println!("args {:?}", flags);
  ExitCode(0)
}
