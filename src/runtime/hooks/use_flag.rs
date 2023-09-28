use std::collections::HashMap;

use super::FlagError;

#[derive(Debug, Clone, PartialEq)]
/// FlagData
pub struct FlagData(Option<String>);

impl TryInto<bool> for FlagData {
  type Error = FlagError;
  fn try_into(self) -> Result<bool, Self::Error> {
    match self.0 {
      Some(value) => match value.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(FlagError::InvalidFormat),
      },
      None => Err(FlagError::Unexisting),
    }
  }
}

impl TryInto<i32> for FlagData {
  type Error = FlagError;
  fn try_into(self) -> Result<i32, Self::Error> {
    match self.0 {
      Some(value) => value.parse::<i32>().map_err(|_| FlagError::ParseIntError),
      None => Err(FlagError::Unexisting),
    }
  }
}

impl TryInto<String> for FlagData {
  type Error = FlagError;
  fn try_into(self) -> Result<String, Self::Error> {
    match self.0 {
      Some(value) => Ok(value),
      None => Err(FlagError::Unexisting),
    }
  }
}
impl FlagData {
  /// Check is the flag value is empty
  pub fn is_empty(self) -> bool {
    self.0.is_some_and(|value| value.is_empty())
  }
}
/// Using flag
pub fn use_flag(
  name: &'static str,
  short: Option<char>,
  flags: &HashMap<String, String>,
) -> FlagData {
  let contains_name = flags.contains_key(&name.to_string());
  let contains_short =
    if let Some(short) = short { flags.contains_key(&short.to_string()) } else { false };

  let to_return = match (contains_name, contains_short) {
    (false, false) => None,
    (true, _) => {
      let value: Option<&String> = flags.get(&name.to_string());
      value.cloned()
    }
    (_, true) => {
      let value: Option<&String> = flags.get(&short.unwrap().to_string());
      value.cloned()
    }
  };

  FlagData(to_return)
}

#[test]
fn test_use_flag() {
  use std::collections::HashMap;

  let mut args = HashMap::new();

  args.insert("name".to_string(), "test".to_string());
  let flag = use_flag("name", Some('n'), &args);
  assert_eq!(flag, FlagData(Some("test".to_string())));

  let mut args = HashMap::new();

  args.insert("n".to_string(), "test".to_string());
  let flag = use_flag("name", Some('n'), &args);
  assert_eq!(flag, FlagData(Some("test".to_string())));
}
