use clier_parser::Argv;

fn main() {
  let parsed = Argv::parse();
  println!("{:#?}", parsed);
}
