use alloc::{boxed::Box, vec::Vec};

pub fn remove_dashdash<'a>(args: &[&'a str]) -> (Vec<&'a str>, Box<str>) {
  let mut encountered_dash_dash = false;
  let mut after_double_dash = Vec::new();

  let args: Vec<&str> = args
    .iter()
    .filter(|arg| match (**arg == "--" && !encountered_dash_dash, encountered_dash_dash) {
      (true, _) => {
        encountered_dash_dash = true;
        false
      }
      (false, true) => {
        after_double_dash.push(" ");
        after_double_dash.push(arg);
        false
      }
      _ => true,
    })
    .cloned()
    .collect();

  if !after_double_dash.is_empty() {
    after_double_dash.remove(0);
  }

  let test = after_double_dash.join("");

  (args, test.into())
}

pub fn is_short_flag(flag: &str) -> bool {
  flag.starts_with('-') && !flag.starts_with("--")
}

pub fn is_long_flag(flag: &str) -> bool {
  flag.starts_with("--") && flag[2..].len() > 1
}

pub fn strip_dash(flag: &str) -> &str {
  let prefix = if is_long_flag(flag) { "--" } else { "-" };
  flag.strip_prefix(prefix).expect("Unable to strip dash")
}

#[test]
fn test_dashash() {
  let result =
    remove_dashdash(&["command", "--flag=value", "--no-value", "--", "something", "othershit"]);

  assert!(&*result.1 == "something othershit");
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

  assert_eq!(result, "valid");
}
