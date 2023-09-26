use super::utils::{is_long_flag, is_short_flag, strip_dash};

pub fn parse_flag(flag: String, next_arg: Option<&String>) -> (String, String) {
  let has_equal = flag.contains('=');

  let flag_vec = flag.split('=').collect::<Vec<&str>>();
  let key = flag_vec.first().unwrap().to_string();

  let is_long = is_long_flag(key.clone());

  let key = strip_dash(is_long, key).unwrap();

  let (key, value) = if has_equal {
    let value = flag_vec.get(1).unwrap().to_string();

    (key, value)
  } else {
    let case_to_fail = &"--tt".to_string();

    let next_arg = next_arg.unwrap_or(case_to_fail);
    let is_next_flag = is_short_flag(next_arg) || is_long_flag(next_arg);
    let value = if is_next_flag {
      if key.starts_with("no-") {
        "false".to_string()
      } else {
        "true".to_string()
      }
    } else if key.starts_with("no-") {
      "false".to_string()
    } else {
      next_arg.to_string()
    };

    match key.strip_prefix("no-") {
      Some(key_no_no) => (key_no_no.to_string(), value),
      None => (key.to_string(), value),
    }
  };

  (key, value)
}
