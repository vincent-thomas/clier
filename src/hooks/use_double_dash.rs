use crate::ClierV2;

/// .
#[derive(Debug)]
pub struct DoesntExist;
/// .
pub fn use_double_dash(clier: &ClierV2) -> Result<String, DoesntExist> {
  if clier.argv.after_dashes().is_empty() {
    return Err(DoesntExist);
  }
  Ok(clier.argv.after_dashes().to_string())
}
