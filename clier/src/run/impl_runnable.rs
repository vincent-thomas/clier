// use console::Term;

// use crate::{
//   builder::{CmdArgs, Handler, RCommand},
//   prelude::*,
//   AlreadyHasMeta, Clier,
// };

// use crate::run::resolver::{flag_resolver, resolve_command, Action};
// use crate::run::{help::help, ExitCode, Runnable};

// impl R

// impl Runnable for Clier<AlreadyHasMeta> {
//   fn command(mut self, cmd: RCommand) -> Self {
//     self.registered_commands.push(cmd);
//     self
//   }

//   fn root(self, description: &str, handler: Handler) -> Self {
//     // let options = self.clone().options.0;
//     let root_command = RCommand::new("root", description, handler);

//     // if let Some(usage) = options.usage {
//     //   root_command = root_command.usage(usage.as_str());
//     // }
//     self.command(root_command)
//   }

//   fn commands(mut self, cmd: Vec<RCommand>) -> Self {
//     self.registered_commands = cmd;
//     self
//   }

//   fn run(self) -> Result<ExitCode, Error> {
//     let what_to_do = resolve_command(&self.args, &self.registered_commands);

//     match what_to_do {
//       Action::ShowHelp(commands) => {
//         help(&commands, &self.args.commands, self.clone().options.0);
//         Ok(0.into())
//       }
//       Action::ShowVersion => {
//         let term = Term::stdout();
//         let _ = term.write_line(&f!("v{}", self.clone().options.0.version));
//         Ok(0.into())
//       }
//       Action::RunCommand(name, command) => {
//         let registered_flags = flag_resolver(&command.flags.unwrap_or(vec![]), &self.args.flags);

//         if let Ok(flags) = registered_flags {
//           let mut commands = self.args.commands.clone();
//           for _ in 0..name.split('.').count() {
//             commands.remove(0);
//           }

//           let mut args_default = self.args.clone();
//           args_default.commands = commands;
//           args_default.flags = self.args.flags;

//           let exit_code =
//             (command.handler)(CmdArgs { args: args_default, registered_flags: flags }).into();
//           Ok(exit_code)
//         } else {
//           eprintln!("Flag not found: {}", registered_flags.unwrap_err());
//           Ok(1.into())
//         }
//       }
//     }
//   }
// }
