struct CommandMeta {
  name: String,
  description: Option<String>
}



fn main() {
  let app = Clier::new().meta();

  let root = Commands::Root {
    meta: cmd_meta!("testing", "tesintg"),
    children: &[Commands::Command {
      meta: cmd_meta!("tesitng", "tesijfdsj"),
      handler: || 0
    }]
  }

  app.commands(root);
  app.run();
}
