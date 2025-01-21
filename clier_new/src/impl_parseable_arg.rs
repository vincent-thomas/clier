use crate::{ParsableArgument, ParsableArgumentPayload, ParsingError};

impl ParsableArgument for String {
  fn extract(payload: ParsableArgumentPayload) -> Result<Self, ParsingError> {
    match payload {
      ParsableArgumentPayload::KeyNotFound => Err(ParsingError::KeyNotFound),
      ParsableArgumentPayload::OnlyKey { no_prefix } => {
        if no_prefix {
          Err(ParsingError::KeyNotFound)
        } else {
          Err(ParsingError::WrongValueType)
        }
      }
      ParsableArgumentPayload::WithValue(value) => Ok(value),
    }
  }
}

impl ParsableArgument for bool {
  fn extract(payload: ParsableArgumentPayload) -> Result<Self, ParsingError> {
    match payload {
      ParsableArgumentPayload::KeyNotFound => Err(ParsingError::KeyNotFound),
      ParsableArgumentPayload::OnlyKey { no_prefix } => Ok(!no_prefix),
      ParsableArgumentPayload::WithValue(value) => match value.as_str() {
        "true" | "false" => Ok(match value.as_str() {
          "true" | "yes" => true,
          "false" | "no" => false,
          _ => unreachable!(),
        }),
        _ => Err(ParsingError::WrongValueType),
      },
    }
  }
}

impl<T> ParsableArgument for Option<T>
where
  T: ParsableArgument + std::fmt::Debug,
{
  fn extract(payload: ParsableArgumentPayload) -> Result<Self, ParsingError> {
    match payload {
      ParsableArgumentPayload::KeyNotFound => Ok(None),
      ParsableArgumentPayload::OnlyKey { no_prefix } => {
        let inner = T::extract(ParsableArgumentPayload::OnlyKey { no_prefix })?;
        Ok(Some(inner))
      }
      ParsableArgumentPayload::WithValue(value) => {
        let inner = T::extract(ParsableArgumentPayload::WithValue(value))?;
        Ok(Some(inner))
      }
    }
  }
}

macro_rules! impl_parsable_argument_numbers {
    ($($t:ty)*) => {
        $(
            impl ParsableArgument for $t {
                fn extract(payload: ParsableArgumentPayload) -> Result<Self, ParsingError> {

                    match payload {
                      ParsableArgumentPayload::KeyNotFound => Err(ParsingError::KeyNotFound),
                      ParsableArgumentPayload::OnlyKey { no_prefix } => if no_prefix {
                          Err(ParsingError::KeyNotFound)
                      } else {
                          Err(ParsingError::WrongValueType)
                      },
                      ParsableArgumentPayload::WithValue(value) => {
                        Ok(value.to_string().parse().or(Err(ParsingError::WrongValueType))?)
                      }
                    }
                }
            }
        )*
    };
}

impl_parsable_argument_numbers! { u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 isize usize }
