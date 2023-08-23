#[derive(Clone, Copy, Debug)]
pub struct ProgramOptions {
  pub name: &'static str,
  pub version: &'static str,
  pub usage: Option<&'static str>,
  pub description: &'static str
}

#[derive(Debug, Clone)]
pub enum Spacing {
  Dots,
  Space,
  Dot,
  Custom(char)
}

pub struct CommandArgs {
  pub commands: Vec<String>,
  pub flags: Vec<(String, String)>,
  pub conf: ProgramOptions
}