pub fn is_short_flag(flag: impl Into<String>) -> bool {
  let flag = flag.into();
  flag.starts_with('-') && !flag.starts_with("--")
}

pub fn is_long_flag(flag: impl Into<String>) -> bool {
  let flag = flag.into();
  flag.starts_with("--") && flag[2..].len() > 1
}

pub fn strip_dash(flag: impl Into<String>) -> String {
  let flag = flag.into();
  flag
    .strip_prefix(if is_long_flag(flag.clone()) { "--" } else { "-" })
    .map(|v| v.to_string())
    .expect("shiiit")
}

#[test]
fn test_short_flag() {
  let mut flag = "-a";
  assert!(is_short_flag(flag));

  flag = "-at";
  assert!(is_short_flag(flag));

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
  let mut result = strip_dash("--valid");

  assert!(result == "valid");

  result = strip_dash("-valid");

  assert_eq!(result, "valid".to_string());
}
