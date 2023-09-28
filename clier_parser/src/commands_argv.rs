use super::utils::{is_long_flag, is_short_flag};

pub fn filter_commands(index: &usize, command: &str, all_args: &[String]) -> bool {
  let may_flag_index = (if *index == 0 { 1 } else { *index }) - 1;

  let is_current_flag = is_short_flag(command.clone()) || is_long_flag(command.clone());
  let may_flag = all_args.get(may_flag_index);
  let is_before_flag = may_flag.is_some_and(|v| is_short_flag(v) || is_long_flag(v));

  !(is_before_flag && is_current_flag || command.starts_with('-'))
}
