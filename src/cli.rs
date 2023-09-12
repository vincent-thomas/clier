use crate::error::Error;
use crate::format::match_command;
use crate::help;
use crate::{Args, Command};

#[derive(Debug, Clone)]
pub struct CliMeta {
    pub name: String,
    pub description: String,
    pub usage: Option<String>,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct Clier {
    pub options: Option<CliMeta>,
    pub(crate) registered_commands: Vec<Command>,
    pub args: Args,
}

#[derive(Debug)]
pub struct InvalidFormat;

pub trait Runnable {
    fn meta(self, meta: CliMeta) -> Self;
    fn add_command(self, cmd: Command) -> Self
    where
        Self: Sized;
    fn commands(self, cmd: Vec<Command>) -> Self;
    fn run(self) -> Result<i32, Error>;
}

impl Runnable for Clier {
    fn meta(mut self, meta: CliMeta) -> Self {
        self.options = Some(meta);
        self
    }
    fn add_command(mut self, cmd: Command) -> Self {
        if cmd.name.contains('.') {
            panic!(
                "{:?}",
                Error::InvalidFormat(String::from("'name' can't contain '.'",))
            );
        }
        self.registered_commands.push(cmd);
        self
    }

    fn commands(mut self, cmd: Vec<Command>) -> Self {
        self.registered_commands = cmd;
        self
    }

    fn run(self) -> Result<i32, Error> {
        let command_to_run = match_command(&self.registered_commands, &self.args.commands);
        use crate::hooks::use_flag;

        let is_version = use_flag("version", Some('v'), &self.args.flags).try_into();

        if is_version.unwrap_or(false) {
            if self.options.is_none() {
                return Err(Error::NoMeta);
            }
            println!("v{}", self.options.unwrap().version);
            std::process::exit(0);
        }
        let is_help = use_flag("help", Some('h'), &self.args.flags)
            .try_into()
            .unwrap_or(false);
        if is_help || command_to_run.is_none() {
            help(
                &self.registered_commands,
                &self.args.commands,
                self.options.expect("'meta' function is not called"),
            );
            return Ok(0);
        }
        if let Some(command) = command_to_run {
            let exit_code = (command.handler)(self.args);
            Ok(exit_code)
        } else {
            Err(Error::NoCommandAndNoHooks)
        }
    }
}

use crate::format::prepare_vargs;

impl Clier {
    pub fn parse(args: Vec<String>) -> Clier {
        Clier {
            options: None,
            registered_commands: vec![],
            args: prepare_vargs(&args),
        }
    }
}
