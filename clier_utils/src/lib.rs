use quote::{quote, ToTokens, TokenStreamExt};

#[derive(Debug, Clone)]
pub struct MetaValue {
  pub description: String,
  pub short: Option<char>,
  pub long: String,
  pub optional: bool,
}
impl ToTokens for MetaValue {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let MetaValue { description, short, long, optional } = self;

    let desc_tokens = quote! { #description.to_string() };

    let short_tokens = if let Some(short_value) = short {
      quote! { Some(#short_value) }
    } else {
      quote! { None }
    };

    let long_tokens = quote! { #long.to_string() };

    tokens.append_all(quote! {
        MetaValue {
           description: #desc_tokens,
            short: #short_tokens,
            long: #long_tokens,
            optional: #optional,
        }
    });
  }
}
