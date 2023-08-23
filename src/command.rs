use crate::types::Handler;

pub struct RunOptions {
  pub program_name: &'static str,
  pub version: &'static str,
  pub usage: Option<&'static str>,
  pub description: &'static str,
}


#[derive(Debug, Clone)]
pub struct CliCommand {
  name: &'static str,
  help_string: Option<&'static str>,
  handler: Handler,
  description: Option<&'static str>,
  usage: Option<&'static str>
}

#[derive(Debug, Clone)]
pub struct Command {
  pub name: &'static str,
  pub help_string: Option<&'static str>,
  pub handler: Handler,
  pub description: &'static str,
  pub usage: Option<&'static str>
}

impl CliCommand {
  pub fn new(name: &'static str, handler: Handler) -> Self {
    return Self {
      name,
      description: None,
      handler,
      help_string: None,
      usage: None
    }
  }
  pub fn build(self) -> Command {
    Command {
      name: self.name,
      description: self.description.unwrap(),
      handler: self.handler,
      help_string: self.help_string,
      usage: self.usage
    }
  }
  pub fn description(mut self, description: &'static str) -> Self {
    self.description = Some(description);
    self
  }
  pub fn usage(mut self, usage: &'static str) -> Self {
    self.usage = Some(usage);
    self
  }
  pub fn help_string(mut self, string: &'static str) -> Self {
    self.help_string = Some(string);
    self
  }
}