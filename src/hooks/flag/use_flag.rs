use super::FlagError;
use crate::prelude::Flags;

#[derive(Debug, Clone, PartialEq)]
pub struct FlagData {
  pub value: Option<String>,
}

impl TryInto<bool> for FlagData {
  type Error = FlagError;
  fn try_into(self) -> Result<bool, Self::Error> {
    match self.value {
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
    match self.value {
      Some(value) => value.parse::<i32>().map_err(|_| FlagError::ParseIntError),
      None => Err(FlagError::Unexisting),
    }
  }
}

impl TryInto<String> for FlagData {
  type Error = FlagError;
  fn try_into(self) -> Result<String, Self::Error> {
    match self.value {
      Some(value) => Ok(value),
      None => Err(FlagError::Unexisting),
    }
  }
}

impl FlagData {
  pub fn is_empty(self) -> bool {
    self.value.is_some_and(|value| value.is_empty())
  }
}

pub fn use_flag(name: &'static str, short: Option<char>, flags: &Flags) -> FlagData {
  let contains_name = flags.contains_key(&name.to_string());
  let contains_short =
    if let Some(short) = short { flags.contains_key(&short.to_string()) } else { false };

  match (contains_name, contains_short) {
    (false, false) => FlagData { value: None },
    (true, _) => {
      let value: Option<&String> = flags.get(&name.to_string());
      FlagData { value: value.cloned() }
    }
    (_, true) => {
      let value: Option<&String> = flags.get(&short.unwrap().to_string());
      FlagData { value: value.cloned() }
    }
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use super::*;

  #[test]
  fn test() {
    let mut args: Flags = HashMap::new();

    args.insert("name".to_string(), "test".to_string());
    let flag = use_flag("name", Some('n'), &args);
    assert_eq!(flag, FlagData { value: Some("test".to_string()) });
  }
}
