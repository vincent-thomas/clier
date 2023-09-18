use clier::Clier;

fn main() {
  let args = Clier::parse().args;
  println!("{:#?}", args);
}
