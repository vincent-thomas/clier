mod use_flag;
pub use use_flag::*;
mod use_flags;
pub use use_flags::*;

#[derive(Debug, Clone)]
pub enum FlagError {
    InvalidFormat,
    Unexisting,
    ParseIntError,
}

pub trait Transformer {
    fn transform(self) -> Self;
}
