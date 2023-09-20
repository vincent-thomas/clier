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