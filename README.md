# Clier

<!-- cargo-rdme start -->

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

fn main() {
  let args: Argv = Clier::parse().args;
  println!("{:#?}", args);
}

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
soon...

<!-- cargo-rdme end -->
