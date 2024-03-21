use std::env;
use std::path::{Path, PathBuf};
use proc_macro2::TokenStream;
use prost_build::{Service, ServiceGenerator};
use prost_types::FileDescriptorSet;
use quote::{format_ident, quote};

include!("_includes.rs");

pub struct Config {
    pub prost_config: prost_build::Config,
}

impl Default for Config {
    fn default() -> Self {
        let mut prost_config = prost_build::Config::default();
        prost_config
            .service_generator(Box::new(Gen::default()))
            .file_descriptor_set_path(
                PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR environment variable not set"))
                    .join("file_descriptor_set.bin"))
            .include_file("_includes.rs")
            .enable_type_names();
        Self { prost_config }
    }
}

impl Config {
    pub fn compile_fds(&mut self, protos: FileDescriptorSet) -> std::io::Result<()> {
        self.prost_config.compile_fds(protos)
    }

    pub fn compile_protos(&mut self, protos: &[impl AsRef<Path>], includes: &[impl AsRef<Path>]) -> std::io::Result<()> {
        self.prost_config.compile_protos(protos, includes)
    }
}

#[derive(Default)]
struct Gen {}

impl ServiceGenerator for Gen {
    fn generate(&mut self, service: Service, buf: &mut String) {
        let mut svc_gen = ServiceGen::default();
        svc_gen.generate(service);
        let file = syn::parse2(svc_gen.items).unwrap();
        let out = prettyplease::unparse(&file);
        buf.push_str(&out)
    }
}

#[derive(Default)]
struct ServiceGen {
    items: proc_macro2::TokenStream,
}

impl ServiceGen {
    fn add(&mut self, item: proc_macro2::TokenStream) {
        self.items.extend(item);
    }

    fn generate(&mut self, service: Service) {
        let mut is_msg_service = false;
        println!("cargo:warning=Starting generate: {:?}", service);
        service.options.uninterpreted_option.iter().for_each(|option| {
            println!("cargo:warning=OPTION: {:?}", option);
        });

        self.generate_server(service.clone(), is_msg_service);
        self.generate_client(service.clone(), is_msg_service);
    }

    fn generate_client(&mut self, service: Service, is_msg_service: bool) {
        // let ctx_type = if is_msg_service {
        //     quote! { ::cosmossdk_core::Context }
        // } else {
        //     quote! { ::cosmossdk_core::ReadContext }
        // };
        let ctx_type = quote! { ::cosmossdk_core::Context };

        let mut methods = vec![];
        for method in service.methods {
            let name = format_ident!("{}", method.name);
            let input = format_ident!("{}", method.input_type);
            let output = format_ident!("{}", method.output_type);
            methods.push(quote! {
                pub fn #name(&self, ctx: &dyn #ctx_type, req: &#input) -> ::cosmossdk_core::Result<#output> {
                    todo!()
                }
            })
        }
        let client_name = format_ident!("{}Client", service.name);
        let service_name = format!("{}.{}", service.package, service.proto_name);
        self.add(quote! {
            pub struct #client_name {
                conn: ::cosmossdk_core::routing::ClientConnection
            }

            impl #client_name {
                #( #methods )*
            }

            impl ::cosmossdk_core::routing::Client for #client_name {
                fn new(conn: ::cosmossdk_core::routing::ClientConnection) -> Self {
                    Self { conn }
                }

                fn describe(helper: &mut dyn ::cosmossdk_core::routing::ClientDescriptorHelper) -> ::cosmossdk_core::routing::ClientDescriptor {
                    ::cosmossdk_core::routing::ClientDescriptor::ServiceClient(#service_name.to_string())
                }
            }

            impl ::cosmossdk_core::encoding::prost::ProstClient for #client_name {}
        })
    }

    fn generate_server(&mut self, service: Service, is_msg_service: bool) {
        // let ctx_type = if is_msg_service {
        //     quote! { ::cosmossdk_core::module::ModuleContext }
        // } else {
        //     quote! { ::cosmossdk_core::module::ModuleReadContext }
        // };
        let ctx_type =quote! { ::cosmossdk_core::module::ModuleContext };

        let mut methods = vec![];
        for method in service.methods {
            let name = format_ident!("{}", method.name);
            let input = format_ident!("{}", method.input_type);
            let output = format_ident!("{}", method.output_type);
            methods.push(quote! {
                fn #name(&self, ctx: &dyn #ctx_type, req: &#input) -> ::cosmossdk_core::Result<#output>;
            })
        }

        let service_name = format_ident!("{}Server", service.name);
        self.add(quote! {
            pub trait #service_name {
                #(#methods)*
            }
        })
    }
}