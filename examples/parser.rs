use clier::Clier;
use std::env;

fn main() {
    let cli = Clier::parse(env::args().collect());

    // cli.add_command();
    /* Test what happens when providing cli args */
    println!("{:#?}", cli);
}
