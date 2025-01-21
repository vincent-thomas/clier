use crate::{ParsableArgument, ParsableArgumentPayload, ParsingError};

#[derive(Debug)]
pub struct ParseQuery {
  keyword: &'static str,
  char: Option<char>,
}

impl ParseQuery {
  pub fn new(keyword: &'static str) -> Self {
    ParseQuery { keyword, char: None }
  }

  pub fn with_char(mut self, char: char) -> Self {
    self.char = Some(char);
    self
  }
}

fn remove_things_in_index(args: &mut Vec<String>, indexes: &[usize]) {
  for (index_indexes, arg) in args.iter_mut().enumerate() {
    if indexes.contains(&index_indexes) {
      *arg = "--".to_string();
    }
  }
}

pub fn internal_parse_argument<T: ParsableArgument>(
  query: ParseQuery,
  args: &mut Vec<String>,
) -> Result<T, ParsingError> {
  let mut iter = args.clone().into_iter().enumerate().skip(1).peekable();

  // Indexes to remove in the argv's
  let mut vec = vec![];

  while let Some((index, next)) = iter.next() {
    if !(next.starts_with(&format!("--{}", query.keyword))
      || next.starts_with(&format!("--no-{}", query.keyword)))
    {
      continue;
    } else {
      vec.push(index);
    }

    // Here we don't know next == key because it can exist a = in there with the value.
    let this_key_and_maybe_value = next.strip_prefix("--").unwrap();

    let (key_and_value, is_no) = match this_key_and_maybe_value.strip_prefix("no-") {
      Some(v) => (v, true),
      None => (this_key_and_maybe_value, false),
    };

    let value = if let Some((_, value)) = key_and_value.split_once("=") {
      // Here the value is included in the index so another index should not be added to ignore.
      Some(value.to_string())
    } else {
      match iter.peek() {
        Some((_, next_value)) => {
          if !next_value.starts_with("--") {
            let (index, next_value) = iter.next().unwrap();
            vec.push(index);

            Some(next_value.clone())
          } else {
            None
          }
        }
        None => None,
      }
    };

    if value.is_some() && is_no {
      return Err(ParsingError::KeyNotFound);
    }

    remove_things_in_index(args, &vec);

    if is_no {
      return T::extract(ParsableArgumentPayload::OnlyKey { no_prefix: true });
    }
    if let Some(value) = value {
      return T::extract(ParsableArgumentPayload::WithValue(value));
    } else {
      return T::extract(ParsableArgumentPayload::OnlyKey { no_prefix: false });
    }
  }

  assert!(vec.len() == 0);

  remove_things_in_index(args, &vec);

  T::extract(ParsableArgumentPayload::KeyNotFound)
}

//let mut item: Vec<String> = args.to_vec();
//let mut std_formatting = item.iter_mut().flat_map(|x| {
//  if !x.starts_with("-") {
//    return Vec::from_iter([x.as_str()]);
//  }
//  let tes: Vec<&str> = match x.split_once("=") {
//    Some(value) => vec![value.0, value.1],
//    None => vec![x],
//  };
//
//  assert!(tes.len() <= 2);
//  tes
//});
//
//let mut payload = ParsableArgumentPayload::default();
//
//while let Some(ref mut next) = std_formatting.next() {
//  if *next == format!("--{}", query.keyword) || *next == format!("--no-{}", query.keyword) {
//    payload.key_exists = true;
//    if next.starts_with("--no-") {
//      payload.no_prefix = true;
//      *next = format!("--{}", query.keyword).as_str();
//    }
//
//    if let Some(next_value) = std_formatting.next() {
//      if next_value.starts_with("-") {
//        payload.value = None;
//        break;
//      } else {
//        payload.value = Some(next_value);
//      }
//    }
//  }
//}
