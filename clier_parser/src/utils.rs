pub fn remove_dashdash(args: &[String]) -> (Vec<String>, String) {
  let mut encountered_dash_dash = false;
  let mut after_double_dash = "".to_string();
  let args: Vec<String> = args
    .iter()
    .filter(|arg| match (arg.as_str() == "--" && !encountered_dash_dash, encountered_dash_dash) {
      (true, _) => {
        encountered_dash_dash = true;
        false
      }
      (false, true) => {
        after_double_dash.push_str(&format!(" {}", arg.as_str()));
        false
      }
      _ => true,
    })
    .cloned()
    .collect();

  if !after_double_dash.is_empty() {
    after_double_dash.remove(0);
  }

  (args, after_double_dash)
}

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
  let prefix = if is_long_flag(flag.clone()) { "--" } else { "-" };
  flag.strip_prefix(prefix).map(|v| v.to_string()).expect("Unable to strip dash")
}

#[test]
fn test_dashash() {
  let result = remove_dashdash(
    ["command", "--flag=value", "--no-value", "--", "something", "othershit"]
      .iter()
      .map(|v| v.to_string())
      .collect::<Vec<String>>()
      .as_slice(),
  );

  assert!(result.1 == "something othershit");
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
