use clier::command::{CmdArgs, Command};

const NAME: &'static str = "test";
const DESCRIPTION: &'static str = "todo...";

pub fn test_command() -> Command {
  Command::new(NAME, DESCRIPTION, command)
}

fn command(args: CmdArgs) -> i32 {
    println!("Hello World");
    0
}
