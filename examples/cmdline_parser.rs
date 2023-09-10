use clier::Clier;
use std::env;

fn main() {
    let cli = Clier::new().parse(env::args().collect());

    println!("{:#?}", cli);
}
