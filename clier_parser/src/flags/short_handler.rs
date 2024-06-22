use alloc::boxed::Box;
use alloc::vec::Vec;
use itertools::Itertools;

use crate::utils::{is_long_flag, is_short_flag};

fn has_no_value_for_last_flag(index: usize, flag: &str, args: &[&str]) -> bool {
  let contains_equal = flag.contains('=');
  let has_space =
    args.get(index + 1).is_some_and(|value| !is_short_flag(value) && !is_long_flag(value));
  !contains_equal && !has_space
}

fn set_default_value_for_flags<'a>(flags: Vec<&'a str>) -> Vec<(&'a str, &'a str)> {
  // Implement this function with NO standard library so the result matches this comment:
  /* flags.into_iter().map(|value| (*value, "true")).collect() */
  let mut result = Vec::new();
  for value in flags.into_iter() {
    result.push((value, "true"));
  }

  result
}

pub fn short_handler<'a>(
  flag_without_dash: &str,
  index: usize,
  args: &[&'a str],
) -> Vec<(Box<str>, Box<str>)> {
  let default_values_short_flags = set_default_value_for_flags(
    flag_without_dash
      .char_indices()
      .map(|(i, _)| {
        &flag_without_dash[i..i + flag_without_dash[i..].chars().next().unwrap().len_utf8()]
      })
      .collect(),
  );

  if has_no_value_for_last_flag(index, &flag_without_dash, args) {
    return default_values_short_flags
      .iter()
      .map(|(first, last)| ((*first).into(), (*last).into()))
      .collect_vec();
  }

  let keys_value = flag_without_dash.split('=').collect_vec();

  let tmp_keys = keys_value.first().unwrap();

  let keys_in_flag: Vec<&str> =
    tmp_keys.char_indices().map(|(i, c)| &tmp_keys[i..i + c.len_utf8()]).collect();

  // KANSKE FEL
  let last_key = *keys_in_flag.last().unwrap();

  set_default_value_for_flags(keys_in_flag)
    .iter()
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
          return (value.0, *keys_value.get(1).unwrap());
        } else {
          return (value.0, "true");
        }
      }

      (value.0, value.1)
    })
    .map(|(first, value)| (first.into(), value.into()))
    .collect()
}
