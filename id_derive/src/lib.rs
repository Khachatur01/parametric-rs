use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(GenerateId)]
pub fn generate_id_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;

    let expanded = quote! {
        impl #name {
            fn generate() -> Self {
                Self(Id::generate())
            }
        }
    };

    TokenStream::from(expanded)
}