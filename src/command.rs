#[derive(Debug, Clone)]
pub struct Args {
    pub commands: Vec<String>,
    pub flags: Vec<(String, String)>,
}

pub type Handler = fn(args: Args);

#[derive(Debug, Clone)]
pub struct Command {
    pub name: &'static str,
    pub handler: Handler,
    pub usage: Option<&'static str>,
    pub description: &'static str,
    pub children: Option<Vec<Command>>,
}
