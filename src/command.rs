#[derive(Debug, Clone, Default)]
pub struct Args {
    pub commands: Vec<String>,
    pub flags: Vec<(String, String)>,
}

pub type Handler = fn(args: Args) -> i32;

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub(crate) handler: Handler,
    pub usage: Option<String>,
    pub description: String,
    pub children: Option<Vec<Command>>,
}

impl Command {
    pub fn new(name: &str, description: &str, usage: Option<&str>, handler: Handler) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            usage: usage.map(|v| v.to_string()),
            handler,
            children: None,
        }
    }

    pub fn push_cmd(
        mut self,
        name: &str,
        description: &str,
        usage: Option<&str>,
        handler: Handler,
    ) -> Self {
        self.children
            .as_mut()
            .unwrap_or(&mut vec![])
            .push(Self::new(name, description, usage, handler));
        self
    }
}
