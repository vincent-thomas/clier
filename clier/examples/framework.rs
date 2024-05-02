use clier::run::ExitCode;
use clier::{CliMeta, Clier, CmdCollection, CmdMeta, Commands};
fn main() {
  let clier_builder = Clier::parse().meta(CliMeta {
    name: "example-clier".into(),
    usage: Some("[command]".into()),
    description: "testing".into(),
    version: Some((0, 0, 0))
  });

  let app = clier_builder.runnable(vec![Commands::Collection(CmdCollection {
    meta: CmdMeta::new("testing", "testing"),
    children: Box::from([
      Commands::Command {
        meta: CmdMeta::new("testchild", "testing"),
        handler: |_| {
          println!("hello");
          ExitCode(0)
        }
      },
      Commands::Collection(CmdCollection {
        meta: CmdMeta::new("testing", "testind"),
        children: Box::from([])
      })
    ])
  })]);

  app.run();
}
