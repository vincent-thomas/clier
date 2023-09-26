pub fn is_short_flag(flag: impl Into<String>) -> bool {
  let flag = flag.into();
  flag.starts_with('-') && flag[1..].len() == 1
}

pub fn is_long_flag(flag: impl Into<String>) -> bool {
  let flag = flag.into();
  flag.starts_with("--") && flag[2..].len() != 1
}

pub fn strip_dash(is_long: bool, flag: impl Into<String>) -> Option<String> {
  let flag = flag.into();
  flag.strip_prefix(if is_long { "--" } else { "-" }).map(|v| v.to_string())
}
