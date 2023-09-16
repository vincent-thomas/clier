use crate::hooks::Flag;

#[derive(Debug, Clone)]
pub struct CmdArgs {
    pub commands: Vec<String>,
    pub flags: Vec<(String, String)>,
    pub registered_flags: Vec<(String, Flag)>,
}

pub type Handler = fn(args: CmdArgs) -> i32;

/// The Command struct to initialize a new command
#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub(crate) handler: Handler,
    pub usage: Option<String>,
    pub flags: Option<Vec<Flag>>,
    pub description: String,
    pub children: Option<Vec<Command>>,
}

impl Command {
    pub fn new(name: &str, description: &str, handler: Handler) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            flags: None,
            usage: None,
            handler,
            children: None,
        }
    }

    pub fn usage(mut self, usage: &str) -> Self {
        self.usage = Some(usage.to_string());
        self
    }

    pub fn flags(mut self, flags: Vec<Flag>) -> Self {
        self.flags = Some(flags);
        self
    }

    pub fn push_cmd(
        mut self,
        name: &str,
        description: &str,
        usage: Option<&str>,
        handler: Handler,
    ) -> Self {
        let mut new_command = Self::new(name, description, handler);

        if let Some(usage) = usage {
            new_command = new_command.clone().usage(usage);
        }

        self.children.as_mut().unwrap_or(&mut vec![]).push(new_command);
        self
    }
}
