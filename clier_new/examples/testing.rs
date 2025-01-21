pub use clier_new::prelude::*;

#[derive(Parser, Debug)]
struct Testing {
  testing: String,
  another: bool,
}

fn main() {
  let testing = Testing::parse();

  println!("{:#?}", testing);
}
