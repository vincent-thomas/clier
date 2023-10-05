mod long_handler;
mod short_handler;
use crate::utils::{is_long_flag, is_short_flag, strip_dash};
use long_handler::long_flag_handler;
use short_handler::short_handler;
use std::collections::HashMap;

fn filter_flag(flag: &str) -> bool {
  let is_valid_flag = is_long_flag(flag) || is_short_flag(flag);

  let has_equal = flag.contains('=');
  let flag_vec = flag.split('=').collect::<Vec<&str>>();
  let key = flag_vec.first().unwrap();
  let has_invalid_no = has_equal && key.starts_with("no-");
  is_valid_flag && !has_invalid_no && flag_vec.len() <= 2
}

pub fn transform_flags_argv(args: &[String]) -> HashMap<String, String> {
  let parsed = args
    .iter()
    .enumerate()
    .filter(|(_, flag)| filter_flag(flag.as_str()))
    .flat_map(|(index, flag)| -> Vec<(String, String)> {
      let next_arg = args.get(index + 1);
      let key_and_value = strip_dash(flag);

      if is_long_flag(flag) {
        long_flag_handler(flag.clone(), next_arg)
      } else {
        short_handler(key_and_value, index, args)
      }
    })
    .collect::<Vec<(String, String)>>();

  HashMap::from_iter(parsed)
}
