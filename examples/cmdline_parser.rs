use clier::Clier;
use std::env;

fn main() {
    let cli = Clier::new().parse(env::args().collect());
    /* Test what happens when providing cli args */
    println!("{:#?}", cli);
}
