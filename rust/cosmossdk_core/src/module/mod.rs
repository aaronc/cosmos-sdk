use crate::routing::{Client, ClientFactory, ModuleServiceResolver, ServiceHandler};

pub trait Module: ModuleServiceResolver {
    // type Config;

    fn describe<T: DescribeModule>(describe: &mut T) -> ModuleDescriptor;

    // fn describe(descriptor: &mut crate::types::cosmos::core::v1alpha1::bundle::ModuleInitDescriptor) -> zeropb::Result<()>;
    // fn route(&self, route_id: u64, ctx: &mut Context, req: *mut u8, res: *mut *mut u8) -> Code;
    fn new<'a, F: ClientFactory<'a>>(config_bytes: &[u8], client_factory: &'a F) -> Self;
}

pub enum ModuleDescriptor {
    Module(String),
    AccountHandler(String),
    InterfaceImpl(String)
}

pub trait DescribeModule {
    fn describe_service<T:ServiceHandler>();
    fn describe_client<'a, T:Client<'a>>();
}
