use crate::{AgentId, Context, ModuleId, ReadContext};
use crate::routing::{Client, ClientFactory, ModuleServiceResolver, ServiceHandler};

mod handler;

pub use handler::*;

pub trait ModuleReadContext: ReadContext {
    fn module_id(&self) -> &ModuleId;
}

pub trait ModuleContext: Context + ModuleReadContext {}

pub trait Module: ModuleServiceResolver {
    // type Config;

    fn describe<T: DescribeModule>(describe: &mut T) -> ModuleDescriptor;

    // fn describe(descriptor: &mut crate::types::cosmos::core::v1alpha1::bundle::ModuleInitDescriptor) -> zeropb::Result<()>;
    // fn route(&self, route_id: u64, ctx: &mut Context, req: *mut u8, res: *mut *mut u8) -> Code;
    fn new<F: ClientFactory>(config_bytes: &[u8], client_factory: &F) -> Self;
}

pub struct ModuleDescriptor {
    pub config_type_name: String,
}

pub trait DescribeModule {
    fn describe_service<T: ServiceHandler>();
    fn describe_client<T: Client>();
}
