use crate::utils::{is_long_flag, is_short_flag, strip_dash};

pub fn long_flag_handler(flag: String, next_arg: Option<&String>) -> Vec<(String, String)> {
  let key_and_value = strip_dash(flag);

  if key_and_value.contains('=') {
    let key_value_vec: Vec<&str> = key_and_value.split('=').collect();
    let is_invalid_many_equal = key_value_vec.len() > 2;

    if is_invalid_many_equal || key_and_value.starts_with("no-") {
      return vec![];
    }

    let key = key_value_vec.first().unwrap().to_string();
    let value = key_value_vec.last().unwrap().to_string();
    return vec![(key, value)];
  }

  if key_and_value.starts_with("no-") {
    let key = key_and_value.strip_prefix("no-").unwrap().to_string();
    return vec![(key, "false".to_string())];
  }

  let key = key_and_value;

  if next_arg.is_some_and(|value| is_short_flag(value) || is_long_flag(value)) {
    vec![(key, "true".to_string())]
  } else {
    let value = next_arg.cloned().unwrap_or("true".to_string());
    vec![(key, value)]
  }
}
