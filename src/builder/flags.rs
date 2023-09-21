#[derive(Debug, Clone)]
pub struct Flag {
  pub name: String,
  pub short: Option<char>,
  pub description: String,
  pub value: Option<String>,
}

impl Flag {
  pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
    Flag { name: name.into(), short: None, description: description.into(), value: None }
  }

  pub fn short(mut self, short: char) -> Self {
    self.short = Some(short);
    self
  }
}
