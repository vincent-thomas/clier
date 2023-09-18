use std::collections::HashMap;

use crate::error::Error;

pub(crate) type CResult<T> = Result<T, Error>;

pub(crate) type Flags = HashMap<String, String>;
