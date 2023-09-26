use super::utils::{is_long_flag, is_short_flag};

pub fn filter_commands(index: &usize, command: &str, all_args: &[String]) -> bool {
  let index = (if *index == 0 { 1 } else { *index }) - 1;

  let is_current_flag = is_short_flag(command.clone()) || is_long_flag(command.clone());
  match all_args.get(index) {
    None => !is_current_flag,
    Some(cmd_before) => {
      let is_before_flag = is_short_flag(cmd_before) || is_long_flag(cmd_before);
      let current_flag_value = is_before_flag && !is_current_flag;
      match (current_flag_value, cmd_before.starts_with("--no-"), cmd_before.contains('=')) {
        (true, true, _) => true,
        (true, _, false) => false,
        (_, _, _) => !is_current_flag,
      }
    }
  }
}
