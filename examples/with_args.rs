use std::env::args;

use clier::hooks::{use_double_dash, use_flag};
use clier::Clier;

fn main() {
  let raw_args = args().collect::<Vec<String>>();
  let raw_args = &raw_args[1..];
  let args = Clier::with_args(raw_args);

  // Try changing 'String' to 'bool', or 'i64'
  let example_hook: Result<String, clier::hooks::FlagError> =
    use_flag("testing", Some('t'), &args).try_into();

  // Everything to the right of '--'
  let raw = use_double_dash(&args);

  println!("flag testing: {:#?}\neverything after '--': {:?}", example_hook, raw);
}
