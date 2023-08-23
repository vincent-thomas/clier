use crate::{App, Cli, command::Command, CliBuilder};

use super::format::format_args;



impl CliBuilder {
  pub fn new() -> Self {
    let builder = CliBuilder {
      description: None,
      name: None,
      commands: None,
      usage: None,
      raw_args: None,
      version: None
    };
    builder
  }

  pub fn meta(mut self, name: &str, descr: &str, version: &str, usage: &str) -> Self {
    self.name = Some(name.to_string());
    self.description = Some(descr.to_string());
    self.version = Some(version.to_string());
    self.usage = Some(usage.to_string());
    self
  }

  pub fn command(mut self, cmd: Command) -> Self {
    let mut this = self.commands.unwrap_or(vec![]);
    this.push(cmd);
    self.commands = Some(this);
    self
  }

  pub fn build(mut self, args: Vec<String>) -> Cli {

    self.raw_args = Some(args);
    Cli {
      options: App {
        name: self.name.unwrap(),
        description: self.description.unwrap(),
        usage: self.usage,
        version: self.version.unwrap()
      },
      commands: self.commands.unwrap(),
      args: format_args(&mut self.raw_args.unwrap())
    }
  }
}
