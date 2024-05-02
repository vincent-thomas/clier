# Clier

## Command Line Argument Parser for Rust

`Clier` is a command line argument parser and command framework for rust.

### Parser

To start a new cli projects run:

```console
$ cargo new demo
$ cargo add clier
```

Then define your CLI in `main.rs`:

```rust
use clier::Argv;
use clier::Clier;

  let args: Argv = Clier::parse().args;
  println!("{:#?}", args);

```

And try it out:

```md
$ cargo run -- command subcommand --test=value --no-production --help --try-me=false
Argv {
commands: [
"command",
"subcommand",
],
flags: {
"test": "value",
"production": "false",
"help": "true",
"try-me": "false",
},
}
```

### Framework

```rust
use clier::builder::{CmdArgs, RCommand};
use clier::error;
use clier::hooks::use_flags;
use clier::run::{ExitCode, Meta, Runnable};
use clier::Clier;

fn first_command_handler(args: CmdArgs) -> i32 {
  let flags = use_flags(&args);
  println!("{:?}", flags);
  0
}
fn main() -> Result<ExitCode, error::Error> {
  let clier = Clier::parse();

  let meta = Meta::new("clier-example-framework", "This is the description", "1.0.0");
  let first_command = RCommand::new("first-command", "Command description", first_command_handler)
    .flag("tes", None, "testing")
    .subcommand("name", "descriptin", |_| {
      /* Code goes here */
      0 /* <- Exit code */
    })
    .subcommand("andra", "descriptin", |_| {
      /* Code goes here */
      0
    });

  clier.meta(&meta).command(first_command).run()
}
```
