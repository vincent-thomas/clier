use crate::{format::prepare_vargs, Cli};

#[derive(Debug, Clone)]
pub struct Clier {
    raw_args: Option<Vec<String>>,
}

impl Default for Clier {
    fn default() -> Self {
        Self::new()
    }
}

impl Clier {
    pub fn new() -> Self {
        Clier { raw_args: None }
    }

    pub fn parse(mut self, args: Vec<String>) -> Cli {
        self.raw_args = Some(args);
        Cli {
            options: None,
            registered_commands: vec![],
            args: prepare_vargs(&self.raw_args.unwrap()),
        }
    }
}
