use crate::{format::prepare_vargs, Cli, CliMeta, Command};

#[derive(Debug, Clone)]
pub struct Clier {
    meta: Option<CliMeta>,
    raw_args: Option<Vec<String>>,
    commands: Option<Vec<Command>>,
}

impl Clier {
    pub fn new() -> Self {
        Clier {
            commands: None,
            raw_args: None,
            meta: None,
        }
    }

    pub fn meta(mut self, meta: CliMeta) -> Self {
        self.meta = Some(meta);
        self
    }

    pub fn parse(mut self, args: Vec<String>) -> Cli {
        self.raw_args = Some(args);
        Cli {
            options: self.meta,
            registered_commands: self.commands.unwrap_or(vec![]),
            args: prepare_vargs(&self.raw_args.unwrap()),
        }
    }
}
