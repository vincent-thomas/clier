use attribute_derive::Attribute;
use attribute_derive::FromAttr;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::Type;
use syn::{Data, DeriveInput, Fields};

#[derive(FromAttr, Debug)]
#[attribute(ident = meta)]
#[attribute(error(missing_field = "`{field}` was not specified"))]
struct CollectionAttribute {
  description: Option<String>,
  short: Option<char>,
}

pub fn impl_flag_parser(input: DeriveInput) -> TokenStream {
  let name = &input.ident;

  let fields = match &input.data {
    Data::Struct(data_struct) => match &data_struct.fields {
      Fields::Named(fields_named) => &fields_named.named,
      _ => unimplemented!(),
    },
    _ => unimplemented!(),
  };

  let meta_values = fields.iter().map(|f| {
    let mut description = None;
    let mut short = None;
    let optional;
    if let Type::Path(value) = &f.ty {
      optional = value.path.segments.first().unwrap().ident == "Option";
    } else {
      panic!("Invalid");
    }
    for attr in &f.attrs {
      let test = CollectionAttribute::from_attribute(attr).unwrap();
      if description.is_none() {
        description = test.description.map(|x| x.replace('\n', "\n      "));
      }
      if short.is_none() {
        short = test.short;
      }
    }
    let name = &f.ident;
    let name_str = name.as_ref().unwrap().to_string();

    clier_utils::MetaValue { description, short, long: name_str.clone(), optional }
  });

  let assignments = meta_values.clone().map(|meta_value| {
    let key = syn::Ident::new(&meta_value.long, Span::call_site());

    let long = &meta_value.long.replace('_', "-");

    let if_short_value = match meta_value.short {
      Some(short) => quote! {
        args.get(#short.to_string().as_str())
      },
      None => quote! { None },
    };

    quote! {
      #key: clier::hooks::Flag::new(#long.to_string(), args.get(#long).or(#if_short_value).cloned()).try_into().unwrap_or_else(|e| {println!("{e}"); std::process::exit(1);})
    }
  });

  let assignments2 = meta_values.clone().map(|v| quote! {#v});
  let gen = quote! {
      impl clier::FlagParser for #name {
          fn parse(clierv2: &clier::ClierV2) -> Self {
              let args = clier_parser::Argv::parse().flags;
              let if_okay = #name {
                  #(#assignments),*
              };

              if_okay
          }
      }

      impl #name {
        fn __clier_internal_meta(&self) -> Vec<clier_utils::MetaValue> {
            vec![#(#assignments2),*]
        }
      }
  };

  gen.into()
}
