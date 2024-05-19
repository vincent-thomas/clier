use console::{style, StyledObject, Term};

use super::Displayer;

/// .
#[derive(Clone, Debug)]
pub struct LabelLogger {
  stdout: Term,
  stderr: Term,
}
/// .
///
impl Default for LabelLogger {
  /// New
  fn default() -> Self {
    Self { stdout: Term::stdout(), stderr: Term::stderr() }
  }
}

fn return_unstyled_label(label: &str) -> StyledObject<String> {
  style(format!(" {label} "))
}

impl Displayer for LabelLogger {
  /// Write shit
  fn info(&self, text: &str) {
    let label = return_unstyled_label("INFO").on_blue();
    let text = format!("{label} {text}");
    let _ = self.stdout.write_line(&text);
  }

  fn error(&self, text: &str) {
    let label = return_unstyled_label("ERROR").on_red();
    let text = format!("{label} {text}");
    let _ = self.stderr.write_line(&text);
  }

  fn warn(&self, text: &str) {
    let label = return_unstyled_label("INFO").on_yellow();
    let text = format!("{label} {text}");
    let _ = self.stderr.write_line(&text);
  }
  fn success(&self, text: &str) {
    let label = return_unstyled_label("SUCCESS").on_green();
    let text = format!("{label} {text}");
    let _ = self.stdout.write_line(&text);
  }
}
