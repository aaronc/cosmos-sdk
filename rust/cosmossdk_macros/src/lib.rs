use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::{DeriveInput, Ident, ItemStruct};

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(module))]
struct ModuleArgs {
    name: String,
    services: Vec<Ident>,
}

#[proc_macro_derive(Module, attributes(module, services, module_config, module_id))]
pub fn derive_module(item: TokenStream) -> TokenStream {
    do_derive_module(item.into()).unwrap().into()

}

fn do_derive_module(item: proc_macro2::TokenStream) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut input = syn::parse2::<syn::DeriveInput>(item)?;
    let ident = input.ident.clone();
    let ModuleArgs { name, services } = deluxe::extract_attributes(&mut input)?;

    Ok(quote!(
        impl cosmossdk_core::module::Module for #ident {
            fn describe<T: cosmossdk_core::module::DescribeModule>(describe: &mut T) -> cosmossdk_core::module::ModuleDescriptor {
                cosmossdk_core::module::ModuleDescriptor {
                    config_type_name: #name.to_string(),
                }
            }

            fn new<'a, F: cosmossdk_core::routing::ClientFactory<'a>>(config_bytes: &[u8], client_factory: &'a F) -> Self {
                todo!()
            }
        }

        impl cosmossdk_core::routing::ModuleServiceResolver for #ident {
            fn resolve_service_handler(&self, index: u16) -> &dyn cosmossdk_core::routing::ServiceHandler {
                todo!()
            }
        }
    ))
}

// #[proc_macro_attribute]
// pub fn module(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let args = match deluxe::parse::<ModuleMacroArgs>(attr) {
//         Ok(desc) => desc,
//         Err(e) => return e.into_compile_error().into()
//     };
//
//     let item = syn::parse_macro_input!(item as ItemStruct);
//
//     let mut i = 0u64;
//     let mut match_arms = vec![];
//     for service in args.services {
//         match_arms.push(quote!(
//             #i => <dyn #service as ::cosmossdk_core::Server>::route(self, method_id, ctx, req, res),
//         ));
//         i += 1;
//     }
//
//     let struct_name = item.ident.clone();
//
//     quote!(
//         #item
//
//         // impl ::cosmossdk_core::Module for #struct_name {
//             // fn route(&self, route_id: u64, ctx: &mut Context, req: *mut u8, res: *mut *mut u8) -> ::cosmossdk_core::Code {
//             //     // service id is second to last byte of route id
//             //     let service_id = (route_id >> 8) & 0xffu64;
//             //     // method id is last byte of route id
//             //     let method_id = route_id & 0xffu64;
//             //     match service_id {
//             //         #(#match_arms)*
//             //         _ => ::cosmossdk_core::Code::Unimplemented,
//             //     }
//             // }
//         // }
//     ).into()
// }

#[proc_macro_attribute]
pub fn module_bundle(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

#[proc_macro_attribute]
pub fn cfg_alloc(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
