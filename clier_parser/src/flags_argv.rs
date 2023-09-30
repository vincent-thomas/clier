use crate::utils::{is_long_flag, is_short_flag, strip_dash};
use std::collections::HashMap;

fn long_flag_handler(flag: String, next_arg: Option<&String>) -> Vec<(String, String)> {
  let key_and_value = strip_dash(flag);

  if key_and_value.contains('=') {
    let key_value_vec: Vec<&str> = key_and_value.split('=').collect();
    let is_invalid_many_equal = key_value_vec.len() > 2;

    if is_invalid_many_equal {
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

fn filter_flag(flag: &str) -> bool {
  let is_valid_flag = is_long_flag(flag) || is_short_flag(flag);

  let has_equal = flag.contains('=');
  let flag_vec = flag.split('=').collect::<Vec<&str>>();
  let key = flag_vec.first().unwrap();
  let has_invalid_no = has_equal && key.starts_with("no-");
  is_valid_flag && !has_invalid_no && flag_vec.len() <= 2
}

fn has_no_value_for_last_flag(index: usize, flag: &str, args: &[String]) -> bool {
  let key_and_value = strip_dash(flag);
  let contains_equal = key_and_value.contains('=');
  let has_space =
    args.get(index + 1).is_some_and(|value| !is_short_flag(value) && !is_long_flag(value));
  !contains_equal && !has_space
}

pub fn transform_flags_argv(args: &[String]) -> HashMap<String, String> {
  let parsed = args
    .iter()
    .enumerate()
    // Is valid flag?
    .filter(|(_, flag)| filter_flag(flag.as_str()))
    .flat_map(|(index, flag)| -> Vec<(String, String)> {
      let next_arg = args.get(index + 1);
      let key_and_value = strip_dash(flag);

      if is_long_flag(flag) {
        return long_flag_handler(flag.clone(), next_arg);
      };

      let default_values_short_flags = key_and_value.chars().map(|v| (v, "true".to_string()));

      if has_no_value_for_last_flag(index, flag, args) {
        return default_values_short_flags.map(|(first, last)| (first.to_string(), last)).collect();
      }

      let keys_value: Vec<&str> = key_and_value.split('=').collect();
      let short_keys = keys_value.first().unwrap().chars();

      short_keys
        .clone()
        .map(|value| (value, "true"))
        .map(|value| {
          let mut keys_value = keys_value.clone();
          if keys_value.get(1).is_none() && keys_value.len() == 1 {
            let next_arg = args.get(index + 1);
            let valid =
              next_arg.is_some_and(|value| !(is_long_flag(value) || is_short_flag(value)));

            if valid {
              keys_value.push(next_arg.unwrap());
            }
          }
          let is_last_in_short_keys = short_keys.clone().last().unwrap() == value.0;
          if is_last_in_short_keys {
            if keys_value.get(1).is_some() {
              return (value.0.to_string(), keys_value.get(1).unwrap().to_string());
            } else {
              return (value.0.to_string(), "true".to_string());
            }
          }
          (value.0.to_string(), value.1.to_string())
        })
        .collect()
    })
    .collect::<Vec<(String, String)>>();

  HashMap::from_iter(parsed)
}
