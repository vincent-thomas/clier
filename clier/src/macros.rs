/// Short for generating command with [Command::new](crate::builder::RCommand)
#[macro_export]
macro_rules! cmd {
  ($cmd_name:expr, $desc:expr, $function:expr) => {
    $crate::builder::RCommand::new($cmd_name, $desc, $function)
  };
  ($cmd_name:expr, $desc:expr, $function:expr, $usage:expr) => {
    $crate::builder::RCommand::new($cmd_name, $desc, $function).usage($usage)
  };
}
