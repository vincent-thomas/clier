#[derive(Debug)]
pub enum ClierError {
    InvalidFormat(String),
    NoMeta,
}
