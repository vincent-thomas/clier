use std::error::Error as StdError;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    InvalidFormat(String),
    NoMeta,
    NoCommandAndNoHooks,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidFormat(msg) => write!(f, "InvalidFormat: {}", msg),
            Error::NoMeta => write!(f, "NoMeta"),
            Error::NoCommandAndNoHooks => write!(f, "NoCommandAndNoHooks"),
        }
    }
}

impl StdError for Error {}
