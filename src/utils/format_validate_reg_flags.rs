use std::collections::HashMap;

use crate::{
  command::Command,
  error::Error,
  hooks::Flag,
  prelude::{CResult, Flags},
};

pub fn format_validate_reg_flags(
  argv_flags: &Flags,
  command: &Command,
) -> CResult<Vec<(String, Flag)>> {
  let mut flags_ret: Vec<(String, Flag)> = Vec::new();

  let Some(flags) = command.clone().flags else {
      return CResult::Ok(vec![])
    };

  let mut taken_items: HashMap<String, bool> = HashMap::new();
  let mut error = String::new();
  flags.into_iter().for_each(|flag| {
    for (flag_key, flag_value) in argv_flags.iter() {
      let is_name_matching = flag.name == flag_key.clone().into();
      let is_short_matching =
        flag.short.is_some_and(|flag_short| flag_short.to_string() == flag_key.clone());

      if is_name_matching || is_short_matching {
        let is_taken = taken_items.contains_key(flag_key);
        match is_taken {
          true => continue,
          false => taken_items.insert(flag_key.clone(), true),
        };

        flags_ret.push((
          flag_key.clone(),
          Flag {
            name: flag_key.clone().into(),
            description: flag.description.clone(),
            short: flag.short,
            value: Some(flag_value.clone()),
          },
        ));
      }
    }
    let result = taken_items.get(flag.name.as_ref());
    if result.is_none() {
      error = flag.name.into()
    };
  });
  match error.len() {
    0 => CResult::Ok(flags_ret),
    _ => CResult::Err(Error::MissingFlag(error)),
  }
}
