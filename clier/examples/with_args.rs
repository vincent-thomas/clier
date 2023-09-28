use std::env::args;

use clier::Argv;
use clier::Clier;

fn main() {
  let raw_args = args().collect::<Vec<String>>();
  let raw_args = &raw_args[1..];
  let args: Argv = Clier::with_args(raw_args).args;

  assert_eq!(args, Argv::from(raw_args));
  println!("{:#?}", args);
}
