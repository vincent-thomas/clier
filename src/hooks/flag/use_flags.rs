use super::Transformer;
use crate::command::CmdArgs;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Flag {
  pub name: Box<str>,
  pub short: Option<char>,
  pub description: String,
  pub value: Option<String>,
}

impl Flag {
  pub fn new(name: &'static str, description: String) -> Self {
    Flag { name: name.into(), short: None, description, value: None }
  }

  pub fn short(mut self, short: char) -> Self {
    self.short = Some(short);
    self
  }
}

impl Transformer for HashMap<String, Flag> {
  fn transform(self) -> Self {
    println!("{:?}", self);
    self
  }
}

pub fn use_flags(args: &CmdArgs) -> HashMap<String, String> {
  let mut flags = HashMap::new();
  args.clone().registered_flags.into_iter().map(|test| (test.0, test.1.value.unwrap())).for_each(
    |value| {
      flags.insert(value.0, value.1);
    },
  );

  flags
}
