use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(StructCodec, attributes(sealed))]
pub fn derive_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let expanded = quote! {
        impl cosmos_schema::ReferenceTypeCodec for #name #generics {
            const NAME: &'static str = stringify!(#name);
        }
        // unsafe impl <'a> StructCodec<'a> for #name<'a> {
        //     // const NAME: &'static str = #name;
        // }
    };

    TokenStream::from(expanded)
}
