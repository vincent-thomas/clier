mod impl_parseable_arg;
pub mod parse_argument;
pub mod prelude;

#[derive(Debug)]
pub enum ParsingError {
  WrongValueType,
  ValueDoesntExist,

  KeyNotFound,
}

#[derive(Debug)]
pub enum ParsableArgumentPayload {
  KeyNotFound,
  OnlyKey {
    no_prefix: bool,
  },
  /// With no- prefix
  WithValue(String),
}

pub trait ParsableArgument: Sized {
  fn extract(payload: ParsableArgumentPayload) -> Result<Self, ParsingError>;
}

pub fn parse_argument<T: ParsableArgument>(
  query: parse_argument::ParseQuery,
  args: &mut Vec<String>,
) -> T {
  parse_argument::internal_parse_argument(query, args).unwrap()
}

fn display_empty_arrays_error(unknown_flags: &[&str]) {
  eprintln!("Unknown Flags:");

  for flag in unknown_flags {
    eprintln!("  - {}", flag);
  }
}

pub trait Parser: Sized {
  fn parse() -> Self {
    let mut args: Vec<String> = std::env::args().collect();
    let this = Self::parse_from_str(&mut args).unwrap();

    let args_left: Vec<&str> = args
      .iter()
      .filter(|x| *x != "" && x.starts_with("--") && *x != "--")
      .map(|x| match x[2..].split_once('=') {
        Some((first, _)) => first,
        None => x,
      })
      .collect();

    if !args_left.is_empty() {
      display_empty_arrays_error(&args_left);
      std::process::exit(1);
    }

    this
  }
  fn parse_from_str(args: &mut Vec<String>) -> Result<Self, ()>;
}

pub trait CmdError {}

impl CmdError for () {}

pub trait FromCmdString: Sized {
  type Error: CmdError;
  fn from_request(cmd: &str) -> Result<Self, Self::Error>;
}
