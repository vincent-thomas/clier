mod app;

use app::commands::generate::generate_command;
use app::commands::new::new_command;
use clier::builder;
use clier::error::Error;
use clier::run::{Meta, Runnable};
use clier::Clier;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

fn main() -> Result<clier::run::ExitCode, Error> {
  let meta = Meta::new(NAME, DESCRIPTION, VERSION).usage("THis is the usage");
  let app = Clier::parse().meta(&meta).command(generate_command()).command(new_command());
  app.run()
}
