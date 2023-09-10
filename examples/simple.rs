use clier::error::ClierError;
use clier::hooks::{use_flag, FlagError};
use clier::{CliMeta, Clier, Command, Runnable};
use std::env::args;

fn main() {
    let meta = CliMeta {
        name: "clier-example-simple".to_string(),
        description: "This is the description".to_string(),
        version: "1.0.0".to_string(),
        usage: None,
    };

    let cli = Clier::new().parse(args().collect());

    let output = cli
        .meta(meta)
        .add_command(Command {
            name: "test",
            usage: Some("test"),
            description: "detta är hur man gör",
            handler: |value| {
                let test = use_flag("test", Some('t'), &value.flags);
                let value: Result<bool, FlagError> = test.try_into();
                println!("Test: {:?}", value);
            },
            children: None,
        })
        .run()
        .unwrap();

    std::process::exit(output)
}
