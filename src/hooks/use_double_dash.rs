use clier_parser::Argv;

/// .
#[derive(Debug)]
pub struct DoesntExist;
/// .
pub fn use_double_dash(argv: &Argv) -> Result<String, DoesntExist> {
  if argv.after_dashes().is_empty() {
    return Err(DoesntExist);
  }
  Ok(argv.after_dashes().to_string())
}
