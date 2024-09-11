mod method;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemTrait, TraitItem};

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(account))]
struct Account(String, #[deluxe(flatten)] AccountAttributes);

#[derive(deluxe::ParseMetaItem, Default)]
#[deluxe(default)]
struct AccountAttributes {
    // publish lists traits to publish that don't have the #[publish] attribute,
    // usually because they're in different files
    publish: Vec<String>,
    codec: Option<String>,
}


#[proc_macro_attribute]
pub fn account(_: TokenStream, item: TokenStream) -> TokenStream { item }

#[proc_macro_attribute]
pub fn module(_: TokenStream, item: TokenStream) -> TokenStream { item }

#[proc_macro_attribute]
pub fn publish(_: TokenStream, item: TokenStream) -> TokenStream { item }

#[proc_macro_attribute]
pub fn on_create(_: TokenStream, item: TokenStream) -> TokenStream { item }

#[proc_macro_attribute]
pub fn account_api(_: TokenStream, item: TokenStream) -> TokenStream {
    let item2: proc_macro2::TokenStream = item.clone().into();
    let input = parse_macro_input!(item as ItemTrait);
    let trait_name = &input.ident;
    let client_struct_name = format_ident!("{}Client", trait_name);

    let methods = input.items.iter().filter_map(|ti| {
        match ti {
            TraitItem::Fn(f) => {
                let name = &f.sig.ident;
                let args = &f.sig.inputs;
                let ret = &f.sig.output;
                Some(quote! {
                    fn #name(#args) #ret {
                        todo!()
                    }
                })
            }
            _ => None
        }
    });

    let implemented_methods = input.items.iter().filter_map(|ti| {
        match ti {
            TraitItem::Fn(f) => {
                let name = &f.sig.ident;
                let method_name = format_ident!("{}_implemented", name);
                Some(quote! {
                    fn #method_name(&self, ctx: &::cosmos_core::Context) -> ::cosmos_core::Result<bool> {
                        todo!()
                    }
                })
            }
            _ => None
        }
    });

    let tokens = quote! {
        #item2

        pub struct #client_struct_name (::cosmos_core::Address)

        impl #trait_name for #client_struct_name {
            #(#methods)*
        }

        impl #client_struct_name {
            #(#implemented_methods)*
        }
    };

    tokens.into()
}

#[proc_macro_attribute]
pub fn module_api(_: TokenStream, item: TokenStream) -> TokenStream { item }
