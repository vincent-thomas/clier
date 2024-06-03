use clier::{clier, prelude::*, Parser};
use clier::{command, ExitCode};

fn main() -> ExitCode {
  let mut app = clier!();

  app.register(build::new(&app));

  app.run()
}

#[derive(Parser, Clone, Debug)]
struct Flags {
  #[meta(description = "teshjfkdsklfdsahjfklds", short = 'n')]
  name: Option<String>,
  test: bool,
}

#[command(description = "testing description", flags = Flags)]
fn build(argv: clier_parser::Argv, flags: Flags) -> clier::ExitCode {
  println!("args {:?}", argv);
  println!("flags {:?}", flags.name);
  println!("flags {:?}", flags.test);
  ExitCode(0)
}
