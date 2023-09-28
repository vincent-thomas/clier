use super::utils::{is_long_flag, is_short_flag};

pub fn filter_commands(index: &usize, command: &str, all_args: &[String]) -> bool {
  let may_flag_index = (if *index == 0 { 1 } else { *index }) - 1;

  let _is_current_flag = is_short_flag(command.clone()) || is_long_flag(command.clone());

  let may_flag = all_args.get(may_flag_index);

  let _is_before_flag = may_flag.is_some_and(|v| is_short_flag(v) || is_long_flag(v));

  if _is_before_flag && _is_current_flag || command.starts_with('-') {
    return false;
  }

  // let Some(cmd_before) = all_args.get(index_of_flag) else {
  //   return !is_current_flag;
  // };
  // let is_before_flag = is_short_flag(cmd_before) || is_long_flag(cmd_before);
  // let current_flag_value = is_before_flag && !is_current_flag;

  // if current_flag_value && cmd_before.starts_with("--no-") {
  //   return true;
  // };

  // if current_flag_value && cmd_before.contains('=') {
  //   return false;
  // }

  true

  // !is_current_flag

  // match (current_flag_value, cmd_before.starts_with("--no-"), cmd_before.contains('=')) {
  //   (true, true, _) => true,
  //   (true, _, false) => false,
  //   (_, _, _) => !is_current_flag,
  // }
}
