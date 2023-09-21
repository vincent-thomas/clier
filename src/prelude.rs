pub use crate::error::Error;
use std::collections::HashMap;

pub(crate) type Result<T> = core::result::Result<T, Error>;

pub(crate) type Flags = HashMap<String, String>;

pub use std::format as f;
