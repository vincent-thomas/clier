use crate::Clier;

/// .
#[derive(Debug)]
pub struct DoesntExist;
/// .
pub fn use_double_dash<T, B>(clier: &Clier<T, B>) -> Result<String, DoesntExist> {
  if clier.args.after_dashes().is_empty() {
    return Err(DoesntExist);
  }
  Ok(clier.args.after_dashes().to_string())
}
