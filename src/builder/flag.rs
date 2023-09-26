/// Registered Flag
#[derive(Debug, Clone, Default)]
pub struct RFlag {
  /// Name
  pub name: String,
  /// Short
  pub short: Option<char>,
  /// Description
  pub description: String,
}

/// Flag
#[derive(Debug, Clone)]
pub struct Flag {
  /// Name
  pub name: String,
  /// Description
  pub description: String,
  /// Value
  pub value: Option<String>,
}

impl RFlag {
  /// New
  pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
    let name = name.into();
    let description = description.into();
    Self { name, description, ..Self::default() }
  }
  /// Short
  pub fn short(mut self, short: char) -> Self {
    self.short = Some(short);
    self
  }
}
