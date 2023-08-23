pub struct Flag {
  pub value: Option<String>,
}

pub trait OutputCheck {
  fn is_true(&self) -> bool;
  fn is_false(&self) -> bool;
}

pub trait Transform {
  fn to_bool(self) -> bool;
}

impl OutputCheck for Flag {
  fn is_true(&self) -> bool {
    self.value.clone().unwrap_or("false".to_string()) == "true"
  }
  fn is_false(&self) -> bool {
    self.value.clone().unwrap_or("true".to_string()) == "false"
  }
}

impl Transform for Flag {
  fn to_bool(self) -> bool {
      let value = self.value.unwrap_or("false".to_string());
      value == "false"
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
mod use_flag_test {
  use super::*;
  #[test]
  fn good_input() {
    let result = use_flag("name", &[("name".to_string(), "true".to_string())]);
    assert!(result.value.unwrap() == "true".to_string());

    let result = use_flag("name", &[("name".to_string(), "false".to_string())]);
    assert_eq!(result.value.unwrap(), "false".to_string());

    let result = use_flag("name", &[("name".to_string(), "1234".to_string())]);
    assert!(result.value.unwrap() == "1234".to_string());
  }
  #[test]
  fn bad_input() {
    let result = use_flag("nam", &[("name".to_string(), "true".to_string())]);
    assert!(result.value.is_none());
  }
}

#[cfg(test)]
mod impl_use_flag_test {
  use super::*;
  #[test]
  fn good_input() {
    let base = use_flag("test", &[("test".to_string(), "true".to_string())]);
    assert!(base.is_true());
  }
  #[test]
  fn bad_input() {
    let base = use_flag("test", &[("test".to_string(), "false".to_string())]);
    assert!(base.is_false());
  }
}