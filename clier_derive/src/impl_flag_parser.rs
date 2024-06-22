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
    let mut description = Vec::new();
    let mut short = None;
    let optional;
    if let Type::Path(value) = &f.ty {
      optional = value.path.segments.first().unwrap().ident == "Option";
    } else {
      panic!("Invalid");
    }
    for attr in &f.attrs {
      if attr.path().is_ident("doc") {
        let test = attr.path().get_ident().cloned();
        if let Some(ident) = test {
          let text = ident.span().source_text().clone();
          if let Some(text) = text {
            description.push(text.replace("/// ", ""));
          }
        }
        continue;
      }
      let test = CollectionAttribute::from_attribute(attr).unwrap();

      if short.is_none() {
        short = test.short;
      }
    }
    let name = &f.ident;
    let name_str = name.as_ref().unwrap().to_string();

    clier_utils::MetaValue {
      description: description.join("\n"),
      short,
      long: name_str.clone(),
      optional,
    }
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
      #key: clier::hooks::Flag::new(#long.to_string(), args.get(#long).or(#if_short_value).map(|x| x.clone().into())).try_into().map_err(|e: clier::hooks::FlagError| {error.insert(#long.to_string(),e.clone()); e}).unwrap_or_default()
    }
  });

  let assignments2 = meta_values.clone().map(|v| quote! {#v});
  let gen = quote! {
      impl clier::FlagParser for #name {
          fn parse() -> (Self, std::collections::HashMap<String, clier::hooks::FlagError>) {
            let raw_args: Vec<String> = std::env::args().collect();
            let test = raw_args.join(" ");

              let args = clier_parser::Argv::from(test.as_str()).flags;

              let mut error: std::collections::HashMap<String, clier::hooks::FlagError> = std::collections::HashMap::new();
              let if_okay = #name {
                  #(#assignments),*
              };

              (if_okay, error)
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
