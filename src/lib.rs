use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Path};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(from))]
struct StructAttrs {
    proto: Path,
}

#[proc_macro_derive(FromProtos, attributes(from))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let proto = StructAttrs::from_derive_input(&ast).unwrap().proto;
    let name = &ast.ident;
    let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { ref named, .. }),
        ..
    }) = &ast.data
    {
        named
    } else {
        panic!("Only structs are supported")
    };

    let fields_from_other = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
          #name: other.#name,
        }
    });

    quote! {
      impl From<#proto> for #name {
        fn from(other: #proto) -> Self {
          Self {
            #(#fields_from_other)*
          }
        }
      }
    }
    .into()
}
