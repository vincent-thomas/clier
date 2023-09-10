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

    let cli = Clier::new().meta(meta).parse(args().collect());

    cli.add_command(Command {
        name: "test",
        usage: Some("test"),
        description: "detta är hur man gör",
        handler: |value| {
            let test = use_flag("test", Some('t'), &value.flags);
            println!("Test: {:#?}", test.value);
        },
        children: None,
    })
    .run()
}
