use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Parser)]
pub fn proc_macro_args_parse(input: TokenStream) -> TokenStream {
  let parsed_input = parse_macro_input!(input as DeriveInput);

  impl_args_parse(parsed_input).into()
}

fn impl_args_parse(input: DeriveInput) -> TokenStream2 {
  let ident = input.ident;

  let mut nice = Vec::default();

  match input.data {
    syn::Data::Struct(data_struct) => match data_struct.fields {
      syn::Fields::Named(fields) => {
        for field in fields.named {
          let ident = field.ident.unwrap();
          nice.push(quote::quote! {
              #ident: clier_new::parse_argument(clier_new::parse_argument::ParseQuery::new(stringify!(#ident)), args)
          })
        }
      }
      _ => unimplemented!(),
    },
    _ => unimplemented!(),
  };

  let nice = nice.iter();

  quote::quote! {
      use clier_new::Parser;
      impl clier_new::Parser for #ident {
          fn parse_from_str(args: &mut Vec<String>) -> Result<Self, ()> {
            let this = Self {
                #(#nice),*
            };

            Ok(this)
          }
      }
  }
}
