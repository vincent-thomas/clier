use crate::{format::match_command, help::render_help, hooks::use_flag, Args, Command};

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
pub trait Runnable {
    fn add_command(self, cmd: Command) -> Self;
    fn commands(self, cmd: Vec<Command>) -> Self;
    fn run(self);
}

impl Runnable for Cli {
    fn add_command(mut self, cmd: Command) -> Self {
        self.registered_commands.push(cmd);
        self
    }

    fn commands(mut self, cmd: Vec<Command>) -> Self {
        self.registered_commands = cmd;
        self
    }

    fn run(self) {
        let is_version = use_flag("version", Some('v'), &self.args.flags).is_true();

        if is_version {
            println!(
                "v{}",
                self.options.expect("'meta' function is not called").version
            );
            std::process::exit(0);
        }

        let is_help = use_flag("help", Some('h'), &self.args.flags).is_true();

        let command_to_run = match_command(&self.registered_commands, &self.args.commands);

        if is_help || command_to_run.clone().is_none() {
            render_help(
                &self.registered_commands,
                &self.args.commands,
                self.options.expect("'meta' function is not called"),
            )
        }

        (command_to_run.unwrap().handler)(self.args);
    }
}
