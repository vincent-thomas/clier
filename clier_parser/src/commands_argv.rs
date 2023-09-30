use super::utils::{is_long_flag, is_short_flag};

fn filter_commands(index: usize, all_args: &[String]) -> bool {
  let command = all_args[index].as_str();

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
  let mut should_be_truthy = filter_commands(0, &["command".to_string()]);
  assert!(should_be_truthy);

  let mut should_be_falsy = filter_commands(1, &["--flag".to_string(), "command".to_string()]);
  assert!(!should_be_falsy);

  should_be_truthy = filter_commands(1, &["--no-flag".to_string(), "command".to_string()]);
  assert!(should_be_truthy);

  should_be_falsy = filter_commands(1, &["-vtd".to_string(), "command".to_string()]);
  assert!(!should_be_falsy);
}

pub fn transform_command_argv(args: &[String]) -> Vec<String> {
  args
    .iter()
    .enumerate()
    .filter(|(index, _)| filter_commands(*index, args))
    .map(|v| v.1.clone())
    .collect::<Vec<String>>()
}
