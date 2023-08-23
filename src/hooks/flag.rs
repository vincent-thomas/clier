use crate::core::hooks::core_use_flag;

pub fn use_flag(name: &'static str, args: &[(String, String)]) -> Option<String> {
  core_use_flag(name, args)
}
