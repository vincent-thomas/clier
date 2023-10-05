use itertools::Itertools;
use std::str::Chars;

use crate::utils::{is_long_flag, is_short_flag};

fn has_no_value_for_last_flag(index: usize, flag: &str, args: &[String]) -> bool {
  let contains_equal = flag.contains('=');
  let has_space =
    args.get(index + 1).is_some_and(|value| !is_short_flag(value) && !is_long_flag(value));
  !contains_equal && !has_space
}

fn set_default_value_for_flags(flags: Chars) -> impl Iterator<Item = (char, String)> + '_ {
  flags.into_iter().map(|value| (value, "true".to_string()))
}

pub fn short_handler(
  flag_without_dash: String,
  index: usize,
  args: &[String],
) -> Vec<(String, String)> {
  let default_values_short_flags = set_default_value_for_flags(flag_without_dash.chars());

  if has_no_value_for_last_flag(index, &flag_without_dash, args) {
    return default_values_short_flags
      .map(|(first, last)| (first.to_string(), last.clone()))
      .collect_vec();
  }

  let keys_value = flag_without_dash.split('=').collect_vec();

  let keys_in_flag = keys_value.first().unwrap().chars();
  let last_key = keys_in_flag.clone().last().unwrap();

  set_default_value_for_flags(keys_in_flag)
    .map(|value| {
      let mut keys_value = keys_value.clone();
      if keys_value.get(1).is_none() && keys_value.len() == 1 {
        let next_arg = args.get(index + 1);
        let valid = next_arg.is_some_and(|value| !(is_long_flag(value) || is_short_flag(value)));

        if valid {
          keys_value.push(next_arg.unwrap());
        }
      }
      let is_last_in_short_keys = last_key == value.0;
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
}
