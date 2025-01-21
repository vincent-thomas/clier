use clier_parser::Argv;
use hashbrown::HashMap;
use thiserror::Error;

/// FlagError
#[derive(Debug, Error, Clone)]
pub enum FlagError {
  // FIXME: ty: typen måste bytas från en String, de asslösning
  // NOTE: ty namnet används för att 'type' är ett keyword
  /// .
  #[error("Invalid format for flag --{flag_key}: \n  Type: {ty}")]
  InvalidFormat {
    ///.
    flag_key: String,
    ///.
    ty: String,
  },
  /// .
  #[error("Unexisting flag: {0}")]
  Unexisting(String),
  /// .
  #[error("Failed to parse integer")]
  ParseIntError,
}

#[derive(Debug, Clone, PartialEq)]
/// FlagData
pub struct Flag {
  value: Option<String>,
  key: String,
}

impl TryInto<Option<bool>> for Flag {
  type Error = FlagError;
  fn try_into(self) -> Result<Option<bool>, Self::Error> {
    match self.value {
      Some(value) => match value.as_str() {
        "true" => Ok(Some(true)),
        "false" => Ok(Some(false)),
        _ => Err(FlagError::InvalidFormat { ty: "boolean".to_string(), flag_key: self.key }),
      },
      None => Ok(None),
    }
  }
}

impl TryInto<bool> for Flag {
  type Error = FlagError;
  fn try_into(self) -> Result<bool, Self::Error> {
    match self.value {
      Some(value) => match value.as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(FlagError::InvalidFormat { ty: "boolean".into(), flag_key: self.key }),
      },
      None => Err(FlagError::Unexisting(self.key)),
    }
  }
}

impl TryInto<Option<i32>> for Flag {
  type Error = FlagError;
  fn try_into(self) -> Result<Option<i32>, Self::Error> {
    match self.value {
      Some(value) => Ok(Some(value.parse::<i32>().map_err(|_| FlagError::ParseIntError)?)),
      None => Ok(None),
    }
  }
}

impl TryInto<i32> for Flag {
  type Error = FlagError;
  fn try_into(self) -> Result<i32, Self::Error> {
    match self.value {
      Some(value) => value.parse::<i32>().map_err(|_| FlagError::ParseIntError),
      None => Err(FlagError::Unexisting(self.key)),
    }
  }
}

impl TryInto<Option<String>> for Flag {
  type Error = FlagError;
  fn try_into(self) -> Result<Option<String>, Self::Error> {
    match self.value.as_deref() {
      Some("true" | "false") => {
        Err(FlagError::InvalidFormat { ty: "string".into(), flag_key: self.key })
      }
      Some(value) => Ok(Some(value.to_string())),
      None => Ok(None),
    }
  }
}

impl TryInto<String> for Flag {
  type Error = FlagError;
  fn try_into(self) -> Result<String, Self::Error> {
    match self.value.as_deref() {
      Some("true" | "false") => {
        Err(FlagError::InvalidFormat { ty: "string".into(), flag_key: self.key })
      }
      Some(value) => Ok(value.to_string()),
      None => Err(FlagError::Unexisting(self.key)),
    }
  }
}
impl Flag {
  /// Check is the flag value is empty
  pub fn is_empty(self) -> bool {
    self.value.is_some_and(|value| value.is_empty())
  }
  /// .
  pub fn new(key: String, thing: Option<String>) -> Self {
    Flag { value: thing, key }
  }
}
/// Using flag
pub fn use_flag(name: &'static str, short: Option<char>, argv: &Argv) -> Flag {
  let flags: &HashMap<Box<str>, Box<str>> = &argv.flags;
  let contains_name = flags.contains_key(name);
  let contains_short = if let Some(short) = short {
    let short_str = short.to_string();
    let short_str = short_str.as_str();
    flags.contains_key(short_str)
  } else {
    false
  };

  let value: Option<String> = match (contains_name, contains_short) {
    (false, false) => None,
    (true, _) => {
      let value: Option<Box<str>> = flags.get(name).cloned();
      value.map(|v| v.into())
    }
    (_, true) => {
      let short_str = short.unwrap().to_string();
      let short_str = short_str.as_str();
      let value: Option<Box<str>> = flags.get(short_str).cloned();
      value.map(|v| v.into())
    }
  };

  Flag { key: name.into(), value }
}
