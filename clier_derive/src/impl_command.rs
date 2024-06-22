use crate::CommandInput;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, ItemFn};

fn parse_command(args: TokenStream) -> CommandInput {
  let mut parse_struct: Option<Ident> = None;
  let parser = syn::meta::parser(|meta| {
    if meta.path.is_ident("flags") {
      let ident: syn::Ident = meta.value()?.parse()?;

      parse_struct = Some(ident);

      Ok(())
    } else {
      Err(meta.error("unsupported property"))
    }
  });

  syn::parse::Parser::parse(parser, args.into()).unwrap();

  CommandInput { flag_struct: parse_struct }
}

pub(crate) fn impl_command(
  ast: ItemFn,
  args: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
  let args = parse_command(args.clone());

  let description = ast
    .attrs
    .iter()
    .filter_map(
      |attr| if attr.path().is_ident("doc") { attr.path().get_ident().cloned() } else { None },
    )
    .filter_map(|x| x.span().source_text().clone())
    .map(|x| x.to_string().replace("/// ", ""))
    .collect::<Vec<String>>()
    .join("\n");

  let fn_name = &ast.sig.ident;
  let fn_name_str = fn_name.to_string().replace('_', " ");

  let mod_name = quote::format_ident!("__clier_internal_{}", &fn_name.to_string());

  let flag_struct = args.flag_struct;

  let gen_option_flag_struct = match flag_struct.clone() {
    // FIXME: Parse works double.
    Some(_) => quote! {
      let test: Vec<clier_utils::MetaValue> = self.flag_struct.clone().map(|x| x.__clier_internal_meta()).unwrap_or(vec![]);
      test
    },
    None => quote! {vec![]},
  };

  let gen_fn_call = match &flag_struct {
    Some(ident) => quote! {
      pub fn call_with_flags(argv: &clier::Clier) -> ExitCode {
        let flags = #ident::parse();

        let meta = flags.0.__clier_internal_meta();

        let mut meta_iter = meta.iter();
        // TODO: Lägg till prioritet för FlagError
        let error_flags: Vec<clier::hooks::FlagError> = flags.1.iter().filter(|(key, error)| {
            let value = meta_iter.find(|x| x.long == *key.clone()).expect("Internal clier error: MetaValue of FlagError didn't exixt");
            !value.optional
            }).map(|(i, e)| e).cloned().collect();

        if !error_flags.is_empty() {
          argv.help();
          println!("Errors:");

          error_flags.iter().for_each(|e| println!("{e}"));
          return clier::ExitCode(1);
        } else {
          return #mod_name::#fn_name(argv.argv.clone(), flags.0);
        }
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
      #mod_name::call_with_flags(clier)
    },
    None => quote! {
      #mod_name::call_without_flags(clier.argv.clone())
    },
  };

  let gen_flag_struct = match &flag_struct {
    Some(_) => quote! {{
      let test = #flag_struct::parse();
      Some(test.0)
    }},
    None => quote! {
      None
    },
  };

  let dec_gen_flag_struct = match flag_struct {
    Some(flag_struct) => quote! {
        Option<#flag_struct>
    },

    None => quote! {
      Option<()>
    },
  };

  let gen_struct = quote! {
    #[allow(non_camel_case_types, missing_docs)]
    #[derive(Clone, Debug)]
    pub struct #fn_name {
          flag_struct: #dec_gen_flag_struct,
          flag_errors: Vec<clier::hooks::FlagError>
      }
      impl #fn_name {
        pub fn new(clier: &clier::Clier) -> Self {
            Self { flag_struct: #gen_flag_struct, flag_errors: vec![] }
        }
      }

    impl clier::Command for #fn_name {
      fn name(&self) -> &'static str {#fn_name_str}
      fn description(&self) -> &'static str {#description}
      fn execute(&self, clier: &clier::Clier) -> clier::ExitCode {
        let status_code = #fn_execute_body;
        return status_code;
      }

        fn flags(&self, clier: &clier::Clier) -> Vec<MetaValue> {
          #gen_option_flag_struct
        }
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
