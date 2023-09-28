use super::utils::{is_long_flag, is_short_flag, strip_dash};
pub fn parse_flag(flag: String, next_arg: Option<&str>) -> (String, String) {
  let has_equal = flag.contains('=');

  let flag_vec = flag.split('=').collect::<Vec<&str>>();
  let key = flag_vec.first().unwrap().to_string();

  let is_long = is_long_flag(key.clone());

  let key = strip_dash(is_long, key).unwrap();
  // Kan man ju bara ta nästa argument som värde
  if has_equal {
    let next_arg = flag_vec.get(1).unwrap().to_string();
    if next_arg.is_empty() {
      println!("Can't parse flag: {}", key);
      std::process::exit(1);
    }
    return (key, next_arg);
  };

  // För att om "Some", ignorera nästa argument
  match key.strip_prefix("no-") {
    Some(key) => (key.to_string(), "false".to_string()),
    None => {
      let is_next_flag = next_arg.is_some_and(|value| is_short_flag(value) || is_long_flag(value));
      if is_next_flag {
        (key.to_string(), "true".to_string())
      } else {
        let value = next_arg.unwrap_or("true");
        (key.to_string(), value.to_string())
      }
    }
  }
}
