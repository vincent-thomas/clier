mod app;

use app::commands::generate::generate_command;
use clier::{run::Runnable, CliMeta, Clier};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() -> clier::ExitCode {
    let meta = CliMeta {
        name: NAME.to_string(),
        description: DESCRIPTION.to_string(),
        version: VERSION.to_string(),
        usage: None,
    };

    let cli = Clier::parse();
    match cli.meta(meta).add_command(generate_command()).run() {
        Ok(value) => value,
        Err(value) => {
            println!("Error: {:?}", value);
            std::process::exit(1)
        }
    }
}
