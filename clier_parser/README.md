# Command Line Argument Parser for Rust
`clier_parser` is a command line argument parser for rust.

## Parser
To start a new cli projects run:

```console
$ cargo new demo && cd demo
$ cargo add clier_parser
```
Then define your CLI in `src/main.rs`:

```rust
use std::env::args;
use clier_parser::Argv;

fn main() {
  let args: Vec<String> = args().collect();
  let parsed = Argv::from(args.as_slice());
  println!("{:#?}", parsed);
}
```

And try it out:
```md
$ cargo run -- command subcommand -tfv testing --test=value --no-production --help
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
    "t": "true",
    "f": "true",
    "v": "testing"
  }
 }
 ```