extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{DeriveInput, Ident, ItemFn};
mod impl_command;
use impl_command::impl_command;
mod impl_flag_parser;
use impl_flag_parser::impl_flag_parser;

mod impl_main;
use impl_main::impl_main;

struct CommandInput {
  flag_struct: Option<Ident>,
}

#[proc_macro_derive(Parser, attributes(meta))]
pub fn derive_flag_parser(input: TokenStream) -> TokenStream {
  let input: DeriveInput = syn::parse(input).unwrap();
  impl_flag_parser(input)
}

#[proc_macro_attribute]
pub fn command(args: TokenStream, input: TokenStream) -> TokenStream {
  let ast: ItemFn = syn::parse2(input.into()).unwrap();

  impl_command(ast, args.into()).into()
}

#[proc_macro_attribute]
pub fn main(_: TokenStream, input: TokenStream) -> TokenStream {
  let ast: ItemFn = syn::parse2(input.into()).unwrap();

  impl_main(ast).into()
}
