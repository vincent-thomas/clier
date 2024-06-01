use std::process::Termination;

/// ExitCode is a wrapper around i32 to implement Termination trait
#[derive(Debug, Clone)]
pub struct ExitCode(pub i32);

impl Termination for ExitCode {
  fn report(self) -> std::process::ExitCode {
    std::process::exit(self.0)
  }
}

impl From<i32> for ExitCode {
  fn from(code: i32) -> Self {
    Self(code)
  }
}
