use super::utils::{is_long_flag, is_short_flag};

pub fn filter_commands(index: &usize, command: &str, all_args: &[String]) -> bool {
  if is_long_flag(command) || is_short_flag(command) {
    return false;
  }

  let mut before_arg = all_args.get((if *index == 0 { 1 } else { *index }) - 1).cloned();

  if before_arg.is_none() {
    let value = String::from("placeholder");
    before_arg = Some(value);
  }

  if is_long_flag(before_arg.clone().unwrap()) || is_short_flag(before_arg.unwrap()) {
    return false;
  }

  true
}
