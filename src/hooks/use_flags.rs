use std::collections::HashMap;

use crate::builder::CmdArgs;

pub fn use_flags(args: &CmdArgs) -> HashMap<String, String> {
  let mut flags = HashMap::new();
  args
    .clone()
    .registered_flags
    .into_iter()
    .map(|(flag_name, flag_value)| (flag_name, flag_value.value.unwrap()))
    .for_each(|value| {
      flags.insert(value.0, value.1);
    });

  flags
}
