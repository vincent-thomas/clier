pub struct Flag {
  pub value: Option<String>,
}

pub trait CheckOutput {
  fn is_true(&self) -> bool;
  fn is_false(&self) -> bool;
}

impl CheckOutput for Flag {
  fn is_true(&self) -> bool {
    self.value.clone().unwrap_or("false".to_string()) == "true"
  }
  fn is_false(&self) -> bool {
    self.value.clone().unwrap_or("true".to_string()) == "false"
  }
} 

pub fn use_flag(name: &'static str, args: &[(String, String)]) -> Flag {
    let flag_keys = args.iter().map(|value| {value.0.clone()}).collect::<Vec<String>>();
    let is_there = flag_keys.contains(&name.to_string());
    if !is_there {
        return Flag { value: None };
    }

    let mut index_name: Option<usize> = None;

    flag_keys.iter()
      .filter(|item| item == &name)
      .enumerate()
      .for_each(|(index, _)| {
        index_name = Some(index);
    });
    let selected_flag = args.get(index_name.unwrap()).unwrap().to_owned();

    return Flag {
        value: Some(selected_flag.1),
    };
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn good_input() {
    let result = use_flag("name", &[("name".to_string(), "true".to_string())]);
    assert!(result.value.unwrap() == "true".to_string());
    let result = use_flag("name", &[("name".to_string(), "false".to_string())]);
    assert_eq!(result.value.unwrap(), "false".to_string());
  }
  #[test]
  fn bad_input() {
    let result = use_flag("nam", &[("name".to_string(), "true".to_string())]);
    assert!(result.value.is_none())
  }
}