/// Label
pub mod label;

/// .
pub trait Displayer {
  /// Write shit
  fn info(&self, text: &str);

  /// Write shit
  fn error(&self, text: &str);

  /// .
  fn warn(&self, text: &str);
  /// .
  fn success(&self, text: &str);
}
