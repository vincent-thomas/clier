use crate::CommandInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemFn};

fn parse_command(args: TokenStream) -> CommandInput {
  let mut parse_desc: Option<String> = None;
  let mut parse_struct: Option<Ident> = None;
  let parser = syn::meta::parser(|meta| {
    if meta.path.is_ident("description") {
      let litstr: syn::LitStr = meta.value()?.parse()?;
      parse_desc = Some(litstr.value());

      Ok(())
    } else if meta.path.is_ident("flags") {
      let ident: syn::Ident = meta.value()?.parse()?;

      parse_struct = Some(ident);

      Ok(())
    } else {
      Err(meta.error("unsupported property"))
    }
  });

  syn::parse::Parser::parse(parser, args.into()).unwrap();

  CommandInput { description: parse_desc, flag_struct: parse_struct }
}

pub(crate) fn impl_command(
  ast: ItemFn,
  args: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  let args = parse_command(args.clone());
  let description = match args.description.clone() {
    Some(s) => quote! { Some(#s) },
    None => quote! { None },
  };

  let fn_name = &ast.sig.ident;
  let fn_name_str = fn_name.to_string().replace("_", " ");

  let mod_name = quote::format_ident!("__clier_internal_{}", &fn_name.to_string());

  let flag_struct = args.flag_struct;

  let gen_option_flag_struct = match flag_struct.clone() {
    // FIXME: Parse works double.
    Some(_) => quote! {
    let test: Vec<clier_utils::MetaValue> = self.flag_struct.clone().map(|x| x.__clier_internal_meta()).unwrap_or(vec![]); test },
    None => quote! {vec![]},
  };

  let gen_fn_call = match &flag_struct {
    Some(ident) => quote! {
      pub fn call_with_flags(argv: &clier::ClierV2) -> ExitCode {
        let flags = #ident::parse(&argv);

        #mod_name::#fn_name(argv.argv.clone(), flags)
      }
    },
    None => quote! {
      pub fn call_without_flags(argv: clier_parser::Argv) -> ExitCode {
        #mod_name::#fn_name(argv)
      }
    },
  };

  let fn_execute_body = match &flag_struct {
    Some(_) => quote! {
      #mod_name::call_with_flags(clierv2)
    },
    None => quote! {
      #mod_name::call_without_flags(&clierv2.argv.clone())
    },
  };

  let gen_flag_struct = match &flag_struct {
    Some(_) => quote! {
      Some(#flag_struct::parse(clierv2))
    },
    None => quote! {
      None
    },
  };

  let gen_struct = quote! {
    #[allow(non_camel_case_types, missing_docs)]
    #[derive(Clone)]
    pub struct #fn_name {
          flag_struct: Option<#flag_struct>
      }
      impl #fn_name {
        pub fn new(clierv2: &clier::ClierV2) -> Self {
            Self { flag_struct: #gen_flag_struct }
        }
      }

    impl clier::Command for #fn_name {
      fn name(&self) -> &'static str {#fn_name_str}
      fn description(&self) -> Option<&'static str> {#description}
      fn execute(&self, clierv2: &clier::ClierV2) -> clier::ExitCode {
        let status_code = #fn_execute_body;
        return status_code;
      }

        fn flags(&self, clierv2: &clier::ClierV2) -> Vec<MetaValue> {#gen_option_flag_struct}
    }
  };

  // FIXME: Circular function calls

  let gen_mod = quote! {
    mod #mod_name {
      use super::*;

      #ast

      #gen_fn_call
    }
  };

  let output = quote! {
    #gen_struct
    #gen_mod
  };

  output
}
