use crate::utils::{is_long_flag, is_short_flag, strip_dash};
use alloc::boxed::Box;
use alloc::vec::Vec;

pub fn long_flag_handler<'a>(
  flag: &'a str,
  next_arg: Option<&'a str>,
) -> Vec<(Box<str>, Box<str>)> {
  let key_and_value = strip_dash(flag);

  if key_and_value.contains('=') {
    let key_value_vec: Vec<&str> = key_and_value.split('=').collect();
    let is_invalid_many_equal = key_value_vec.len() > 2;

    if is_invalid_many_equal || key_and_value.starts_with("no-") {
      return alloc::vec![];
    }

    let key = key_value_vec.first().unwrap();
    let value = key_value_vec.last().unwrap();
    return alloc::vec![((*key).into(), (*value).into())];
  }

  if key_and_value.starts_with("no-") {
    let key = key_and_value.strip_prefix("no-").unwrap();
    return alloc::vec![(key.into(), "false".into())];
  }

  let key = key_and_value;

  if next_arg.is_some_and(|value| is_short_flag(value) || is_long_flag(value)) {
    alloc::vec![(key.into(), "true".into())]
  } else {
    let value = next_arg.unwrap_or("true");
    alloc::vec![(key.into(), value.into())]
  }
}
