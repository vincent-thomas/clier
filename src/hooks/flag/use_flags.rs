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

pub fn use_flags(args: &CmdArgs) -> HashMap<String, String> {
  let mut flags = HashMap::new();
  args
    .clone()
    .registered_flags
    .into_iter()
    .map(|(flag_name, flag_value)| (flag_name, flag_value.value.unwrap()))
    .for_each(|value| {
      flags.insert(value.0, value.1);
    });

  flags
}
