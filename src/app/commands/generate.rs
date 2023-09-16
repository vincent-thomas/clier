use clier::command::{CmdArgs, Command};
use clier::hooks::Flag;

pub fn generate_command() -> Command {
    Command::new("generate", "Generates parts of program", command)
        .usage("generate [--flags=value]")
        .flags(vec![Flag::new("test", "testing".to_string())])
}
fn command(_args: CmdArgs) -> i32 {
    // let flags = use_flags(&args);
    // let result = CommandGenerator::generate();
    // println!("{:?}", flags);
    0
}
