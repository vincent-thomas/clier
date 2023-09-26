use std::collections::HashMap;

use crate::builder::{Flag, RFlag};

pub fn flag_resolver(
  registered_flags: &[RFlag],
  flags: &HashMap<String, String>,
) -> Result<Vec<(String, Flag)>, String> {
  let mut error_flag = "";
  let result: Vec<(String, Flag)> = registered_flags
    .iter()
    .filter(|v| {
      let value = flags.get(&v.name).cloned();
      if value.is_none() {
        error_flag = &v.name;
        false
      } else {
        true
      }
    })
    .map(|v| {
      let value = flags.get(&v.name).cloned();
      let value_return = value.map(|value| Flag {
        name: v.clone().name,
        value: Some(value),
        description: v.clone().description,
      });
      (v.name.clone(), value_return.unwrap())
    })
    .collect();

  if !error_flag.is_empty() {
    Err(error_flag.to_string())
  } else {
    Ok(result)
  }
}
