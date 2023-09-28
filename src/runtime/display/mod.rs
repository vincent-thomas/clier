use crate::prelude::*;
use console::{style, StyledObject, Term};

/// Seriousness of the message
pub enum Displayer {
  /// .
  Info,
  /// .
  Error,
  /// .
  Todo,
}

impl Displayer {
  fn format<'a>(self) -> StyledObject<&'a str> {
    match self {
      Self::Info => style(" INFO ").on_blue(),
      Self::Error => style(" ERROR ").on_red(),
      Self::Todo => style(" TODO ").on_yellow(),
    }
  }
  /// Write shit
  pub fn write(self, text: impl Into<String>) {
    let term = Term::stdout();
    let what = f!("{} {}", self.format(), text.into());
    let _ = term.write_line(what.as_str());
  }

  /// Write shit
  pub fn write_err(self, text: impl Into<String>) {
    let term = Term::stderr();
    let what = f!("{} {}", self.format(), text.into());
    let _ = term.write_line(what.as_str());
  }
}
