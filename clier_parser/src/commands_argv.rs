use super::utils::{is_long_flag, is_short_flag};
use alloc::boxed::Box;
use alloc::vec::Vec;

fn filter_commands(index: usize, all_args: &[&str]) -> bool {
  let command = all_args[index];

  let is_arg_flag = is_long_flag(command) || is_short_flag(command);

  if is_arg_flag {
    return false;
  }

  let is_before_arg_flag = index != 0
    && all_args.get(index - 1).is_some_and(|flag| is_long_flag(flag) || is_short_flag(flag));

  if is_before_arg_flag {
    let before_arg = all_args.get(index - 1).unwrap();
    return before_arg.starts_with("--no-");
  }
  true
}

#[test]
fn test_filtering_commands() {
  let mut should_be_truthy = filter_commands(0, &["command"]);
  assert!(should_be_truthy);

  let mut should_be_falsy = filter_commands(1, &["--flag", "command"]);
  assert!(!should_be_falsy);

  should_be_truthy = filter_commands(1, &["--no-flag", "command"]);
  assert!(should_be_truthy);

  should_be_falsy = filter_commands(1, &["-vtd", "command"]);
  assert!(!should_be_falsy);
}

pub fn transform_command_argv(args: &[&str]) -> Vec<Box<str>> {
  args
    .iter()
    .enumerate()
    .filter(|(index, _)| filter_commands(*index, args))
    .map(|v| (*v.1).into())
    .collect::<Vec<Box<str>>>()
}
