use clier::builder::{CmdArgs, RCommand};
use clier::error;
use clier::hooks::use_flags;
use clier::run::{ExitCode, Meta, Runnable};
use clier::Clier;

fn first_command_handler(args: CmdArgs) -> i32 {
  let flags = use_flags(&args);
  println!("{:?}", flags);
  0
}
fn main() -> Result<ExitCode, error::Error> {
  let clier = Clier::parse();

  let meta =
    Meta::new("clier-example-framework", "This is the description", "1.0.0").usage("<command>");
  let first_command = RCommand::new("first-command", "Command description", first_command_handler)
    .flag("tes", None, "testing")
    .subcommand("name", "descriptin", |_| {
      /* Code goes here */
      0 /* <- Exit code */
    })
    .subcommand("andra", "descriptin", |_| {
      /* Code goes here */
      0
    });

  clier.meta(&meta).command(first_command).run()
}
