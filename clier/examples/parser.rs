use clier_parser::Argv;

fn main() {
  let args: Argv = Argv::parse();
  println!("{:#?}", args.commands);

  if args.commands.get(0).unwrap() == "hello" {
    println!("hello this is a command")
  }
}
