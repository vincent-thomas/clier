
use std::env;

use clier::builder::{Cli, CommandBuilder};
use clier::command::{Command, CommandArgs};
use clier::conf::{ProgramOptions, Spacing};
use clier::hooks::{use_flag, CheckOutput};


fn main() {

  let app = Cli::new(env::args().collect())
    .meta(ProgramOptions {
      name: "example_simple",
      description: "Desc",
      usage: None,
      version: "1.0.0"
    })
    .command(Command {
      name: "test",
      description: "description",
      handler: |commands: CommandArgs| {
        let has_test = use_flag("test", &commands.flags).is_true();
        println!("I work {}! More args {:?}", has_test, commands);
      },
      help_string: None,
      usage: Some("testing")
  }).build();

  app.spacing(Spacing::Space).run();
}