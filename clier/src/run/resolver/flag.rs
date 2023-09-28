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

#[test]
fn test_flag_resolver() {
  let registered_flags = vec![
    RFlag { name: "name".to_string(), description: "name".to_string(), short: None },
    RFlag { name: "age".to_string(), description: "age".to_string(), short: Some('a') },
  ];

  let flags = vec![("name".to_string(), "John".to_string()), ("age".to_string(), "20".to_string())]
    .into_iter()
    .collect();

  let result = flag_resolver(&registered_flags, &flags);
  assert!(result.is_ok());
}
