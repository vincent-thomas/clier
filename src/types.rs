
pub type FormattedFlagsInput<'a> = &'a [(String, String)];
pub type FormattedFlagsOutput = Vec<(String, String)>;
use crate::CliApp;
use crate::conf::CommandArgs;

pub type Handler = fn(opt: CommandArgs);

pub struct Flag {
  pub value: Option<String>,
}

trait FlagTrait {
  fn is_truthy(&self) -> bool;
}