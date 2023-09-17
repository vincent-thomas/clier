Command::new("command", "description", |_args| {
  ...
  0 // <-- i32: Exit Code of program
})
.usage("command [usage text]") // Optional
.flags(vec![
  Flag::new("flag-name", "flag description".to_string() /* <-- In help */)
  .short('t') // Optional
])