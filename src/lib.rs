use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FromProtos, attributes(from))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    quote! {
      impl From<B> for #name {
        fn from(b: B) -> Self {
          Self {
            a: b.b,
          }
        }
      }
    }
    .into()
}
