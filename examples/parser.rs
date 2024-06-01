use clier_parser::Argv;

fn main() {
  let args: Argv = Argv::parse();
  println!("{:#?}", args.commands);

  if args.commands.first().unwrap() == "hello" {
    println!("hello this is a command")
  }
}
