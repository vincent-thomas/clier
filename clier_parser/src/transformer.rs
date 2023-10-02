use crate::flags::transform_flags_argv;
use crate::{commands_argv, Argv};
use commands_argv::transform_command_argv;

fn remove_dashdash(args: &[String]) -> (Vec<String>, String) {
  let mut encountered_dash_dash = false;
  let mut after_double_dash = "".to_string();
  let args: Vec<String> = args
    .iter()
    .filter(|arg| match (arg.as_str() == "--", encountered_dash_dash) {
      (true, _) => {
        encountered_dash_dash = true;
        false
      }
      (false, true) => {
        after_double_dash.push_str(format!(" {arg}").as_str());
        false
      }
      _ => true,
    })
    .cloned()
    .collect();

  if !after_double_dash.is_empty() {
    after_double_dash.remove(0);
  }

  (args, after_double_dash)
}

pub fn transform_vargs(args: &[String]) -> Argv {
  let (args, after_double_dash) = remove_dashdash(args);
  let flags = transform_flags_argv(&args);
  let commands = transform_command_argv(&args);

  Argv { commands, flags, after_double_dash }
}
