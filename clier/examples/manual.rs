use clier::{command, Argv, ExitCode};
use clier::{prelude::*, Parser};

// This is a basic example of the clier command line framework for rust
#[clier::main]
fn main(app: &mut clier::Clier) -> ExitCode {
  // This registers the command to the app. This command will run as the function name.
  // For example, if the user provides 'example-program build', this command below will run.
  app.register(build::new(app));
  // Notice, this is fetched based by the function name.
  // The character "_" is translated into " " by the framework so a function named "test_build"
  // will run when user provides example-program test build".

  // This registers the command "test" to this function
  app.register(test::new(app));

  app.run()
}

// The doc comments automatically describes the function in the help message, this is possible by
// the command attribute macro.

/// This would be the command description in the help message
#[command]
fn test(/* The struct Argv is always available */ argv: Argv) -> clier::ExitCode {
  println!("Test Command!");
  println!("args {:?}", argv);
  ExitCode(0)
}

// This is the flags command 'build' requires and is ENFORCED, by clier, so the types here is what
// will be available in the command.
// NOTE: If using the option type, the type is automatically optional and will be showed as such in
// the help message.
#[derive(Parser, Clone, Debug)]
struct Flags {
  /// Another type that is supported
  age: i32,
  /// The description of the name flag viewable in the help message.
  name: Option<String>,
  // A 'short' can be assigned to a property, which means the syntax '-a' is supported instead of
  // "--age"
  //
}

/// build description
#[command(flags = Flags)]
fn build(argv: Argv, flags: Flags) -> clier::ExitCode {
  println!("args {:?}", argv);
  println!("flags {:?}", flags);
  ExitCode(0)
}
