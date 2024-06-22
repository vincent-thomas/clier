use quote::quote;
use syn::ItemFn;

pub(crate) fn impl_main(ast: ItemFn) -> proc_macro2::TokenStream {
  let fn_name = &ast.sig.ident;
  if fn_name != "main" {
    return syn::Error::new_spanned(fn_name, "This macro can only be used on the main function.")
      .to_compile_error();
  }

  let mut fnu = ast.clone();

  fnu.sig.ident = syn::Ident::new("bootstrap", fnu.sig.ident.span());
  let bootstrap = &fnu;

  let gen_run = quote! {
    fn main() -> ExitCode {
      let raw_args: Vec<String> = std::env::args().collect();
      let test = raw_args[1..].join(" ");
      let args = clier_parser::Argv::from(test.as_str());

      let mut clier = clier::Clier::new(
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_DESCRIPTION"),
        args
      ).version(env!("CARGO_PKG_VERSION"));
      bootstrap(&mut clier)
    }

    #bootstrap
  };

  gen_run
}
