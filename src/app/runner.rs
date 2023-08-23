use crate::{Cli, format::Args, use_help};


pub trait Runnable {
  fn run(self);
}

impl Runnable for Cli {
  fn run(self) {
    let string = "".to_string();
    let command = self.args.commands.get(0).unwrap_or(&string);
    let mut commands_missed = 0;
    self.commands.iter().for_each(|cmd| {
      if cmd.name == command {
        (cmd.to_owned().handler)(Args {commands: self.args.commands.clone(), flags: self.args.flags.clone()});
      } else {
        commands_missed += 1;
      }
    });
    if commands_missed == self.commands.len() {
      use_help(self.commands.clone(), self.options.clone());
    }
  }
}