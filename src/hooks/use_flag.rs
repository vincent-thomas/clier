#[derive(Debug, Clone, PartialEq)]
pub struct Flag {
    value: Option<String>,
}

#[derive(Debug)]
pub enum FlagError {
    InvalidFormat,
    Unexisting,
    ParseIntError,
}

impl TryInto<bool> for Flag {
    type Error = FlagError;
    fn try_into(self) -> Result<bool, Self::Error> {
        match self.value {
            Some(value) => match value.as_str() {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => Err(FlagError::InvalidFormat),
            },
            None => Err(FlagError::Unexisting),
        }
    }
}
impl TryInto<i32> for Flag {
    type Error = FlagError;
    fn try_into(self) -> Result<i32, Self::Error> {
        match self.value {
            Some(value) => value.parse::<i32>().map_err(|_| FlagError::ParseIntError),
            None => Err(FlagError::Unexisting),
        }
    }
}

impl TryInto<String> for Flag {
    type Error = FlagError;
    fn try_into(self) -> Result<String, Self::Error> {
        match self.value {
            Some(value) => Ok(value),
            None => Err(FlagError::Unexisting),
        }
    }
}

impl Flag {
    pub fn is_empty(self) -> bool {
        self.value.is_some() && self.value.unwrap() == ""
    }
}

pub fn use_flag(name: &'static str, short: Option<char>, args: &[(String, String)]) -> Flag {
    let flag_keys = args
        .iter()
        .map(|value| value.0.clone())
        .collect::<Vec<String>>();
    let is_there_key = flag_keys.contains(&name.to_string());
    let is_there_short = if let Some(short_char) = short {
        flag_keys.contains(&short_char.to_string())
    } else {
        false
    };
    if !is_there_key && !is_there_short {
        return Flag { value: None };
    }

    let mut index_name: Option<usize> = None;

    flag_keys
        .iter()
        .filter(|item| {
            item == &name
                || if let Some(short_char) = short {
                    item.to_string() == short_char.to_string()
                } else {
                    false
                }
        })
        .enumerate()
        .for_each(|(index, _)| {
            if index_name.is_none() {
                index_name = Some(index);
            }
        });
    let selected_flag = args.get(index_name.unwrap()).unwrap().to_owned();

    Flag {
        value: Some(selected_flag.1),
    }
}
