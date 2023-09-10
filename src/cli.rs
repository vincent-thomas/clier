use crate::error::ClierError;
use crate::format::match_command;
use crate::help;
use crate::hooks::use_flag;
use crate::{Args, Command};

#[derive(Debug, Clone)]
pub struct CliMeta {
    pub name: String,
    pub description: String,
    pub usage: Option<String>,
    pub version: String,
}

#[derive(Debug, Clone)]
pub struct Cli {
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
    fn run(self) -> Result<i32, ClierError>;
}

impl Runnable for Cli {
    fn meta(mut self, meta: CliMeta) -> Self {
        self.options = Some(meta);
        self
    }
    fn add_command(mut self, cmd: Command) -> Self {
        if cmd.name.contains('.') {
            panic!(
                "{:?}",
                ClierError::InvalidFormat(String::from("'name' can't contain '.'",))
            );
        }
        self.registered_commands.push(cmd);
        self
    }

    fn commands(mut self, cmd: Vec<Command>) -> Self {
        self.registered_commands = cmd;
        self
    }

    fn run(self) -> Result<i32, ClierError> {
        let is_version = use_flag("version", Some('v'), &self.args.flags).into_bool();

        if is_version {
            if self.options.is_none() {
                return Err(ClierError::NoMeta);
            }
            println!("v{}", self.options.unwrap().version);
            std::process::exit(0);
        }

        let is_help = use_flag("help", Some('h'), &self.args.flags).into_bool();
        let command_to_run = match_command(&self.registered_commands, &self.args.commands);

        if is_help || command_to_run.clone().is_none() {
            help(
                &self.registered_commands,
                &self.args.commands,
                self.options.expect("'meta' function is not called"),
            )
        }

        (command_to_run.unwrap().handler)(self.args);
        Ok(0)
    }
}
