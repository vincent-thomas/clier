use clier::Argv;
use clier::Clier;

fn main() {
  let args: Argv = Clier::parse().args;
  println!("{:#?}", args);
}
