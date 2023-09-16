use clier::Clier;

fn main() {
    let cli = Clier::parse().args;

    println!("{:#?}", cli);
}
