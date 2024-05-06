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
use clier_parser::Argv;

let args: Argv = Argv::parse();
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
use clier::run::ExitCode;
use clier::{CliMeta, Clier, CmdCollection, CmdMeta, Commands};
fn main() {
  let clier_builder = Clier::parse().meta(CliMeta {
    name: "example-clier".into(),
    usage: Some("[command]".into()),
    description: "testing".into(),
    version: Some((0, 0, 0))
  });

  let app = clier_builder.runnable(vec![Commands::Collection(CmdCollection {
    meta: CmdMeta::new("testing", "testing"),
    children: Box::from([Commands::Command {
      meta: CmdMeta::new("testchild", "testing"),
      handler: |_| {
        println!("hello");
        ExitCode(0)
      }
    }])
  })]);

  app.run();
}


```
