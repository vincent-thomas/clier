use std::env::args;

use clier_parser::Argv;

fn main() {
  let args: Vec<String> = args().collect();
  let parsed = Argv::from(args.as_slice());
  println!("{:#?}", parsed);
}
