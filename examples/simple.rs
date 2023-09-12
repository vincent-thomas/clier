use clier::hooks::use_flag;
use clier::{CliMeta, Clier, Command, Runnable};
use std::env::args;

fn main() {
    let meta = CliMeta {
        name: "clier-example-simple".to_string(),
        description: "This is the description".to_string(),
        version: "1.0.0".to_string(),
        usage: None,
    };

    let cli = Clier::parse(args().collect());

    let _: Result<i32, clier::error::Error> = cli
        .meta(meta)
        .add_command(Command::new(
            "test",
            "detta är hur man gör",
            Some("test"),
            |value| {
                let test = use_flag("test", Some('t'), &value.flags)
                    .try_into()
                    .unwrap_or_else(|_| "".to_string());
                println!("Test: {:?}", test);
                0
            },
        ))
        .run();
}
