use crate::app::format::Args;

pub type Handler = fn(args: Args);

#[derive(Debug, Clone, Copy)]
pub struct Command {
  pub name: &'static str,
  pub help_string: Option<&'static str>,
  pub handler: Handler,
  pub description: &'static str,
  pub usage: Option<&'static str>
}