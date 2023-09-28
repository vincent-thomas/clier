pub fn is_short_flag(flag: impl Into<String>) -> bool {
  let flag = flag.into();
  flag.starts_with('-') && flag[1..].len() == 1
}

pub fn is_long_flag(flag: impl Into<String>) -> bool {
  let flag = flag.into();
  flag.starts_with("--") && flag[2..].len() > 1
}

pub fn strip_dash(is_long: bool, flag: impl Into<String>) -> Option<String> {
  let flag = flag.into();
  flag.strip_prefix(if is_long { "--" } else { "-" }).map(|v| v.to_string())
}

#[test]
fn test_short_flag() {
  let mut flag = "-a";
  assert!(is_short_flag(flag));

  flag = "-at";
  assert!(!is_short_flag(flag));

  flag = "--t";
  assert!(!is_short_flag(flag));
}

#[test]
fn test_long_flag() {
  let mut flag = "--valid";
  assert!(is_long_flag(flag));

  flag = "--n";
  assert!(!is_long_flag(flag));
}

#[test]
fn test_strip_dash() {
  let mut result = strip_dash(true, "--valid");

  assert!(result == Some("valid".to_string()));

  result = strip_dash(false, "-valid");

  assert_eq!(result, Some("valid".to_string()));
}
