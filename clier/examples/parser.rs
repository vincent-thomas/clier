use clier::Clier;
use clier_parser::Argv;

fn main() {
  let args: Argv = Clier::parse().args;
  println!("{:#?}", args);
}
