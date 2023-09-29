#[test]
fn test_flag_resolver() {
  use crate::{builder::RFlag, run::resolver::flag_resolver};

  let registered_flags = vec![
    RFlag { name: "name".to_string(), description: "name".to_string(), short: None },
    RFlag { name: "age".to_string(), description: "age".to_string(), short: Some('a') },
  ];

  let flags = vec![("name".to_string(), "John".to_string()), ("age".to_string(), "20".to_string())]
    .into_iter()
    .collect();

  let result = flag_resolver(&registered_flags, &flags);
  assert!(result.is_ok());
}
