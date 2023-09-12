mod app;

use app::commands::generate::generate_command;
use clier::{CliMeta, Clier, Runnable};
use std::env::args;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let meta = CliMeta {
        name: NAME.to_string(),
        description: DESCRIPTION.to_string(),
        version: VERSION.to_string(),
        usage: None,
    };

    let cli = Clier::parse(args().collect());
    let result = cli.meta(meta).add_command(generate_command()).run();
    match result {
        Ok(output) => std::process::exit(output),
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
}
