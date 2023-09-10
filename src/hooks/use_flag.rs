use std::fmt;

#[derive(Debug)]
pub struct Flag {
    value: Option<String>,
}

#[derive(Debug)]
pub enum FlagError {
    InvalidFormat,
    Unexisting,
}

impl Flag {
    pub fn into_bool(self) -> bool {
        self.try_into().unwrap_or(false)
    }
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

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value.clone().unwrap_or("Empty".to_string()))
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
