use crate::{AgentId, Context, ModuleId, ReadContext, ServerRequestWrapper};
use crate::routing::{Client, ClientDescriptor, ModuleServiceResolver, ServerRequestImpl, ServerRequestWrapperImpl, Service, ServiceDescriptor, ServiceHandler};

mod handler;

pub use handler::*;

pub trait ModuleReadContext: ReadContext {
    fn module_id(&self) -> &ModuleId;
}

pub trait ModuleContext: Context + ModuleReadContext {}

//
// impl <'a> ModuleContextData<'a> {
//     pub fn new(context: &'a ContextData) -> crate::Result<Self> {
//         let AgentId::Module(module_id) = &context.target else {
//             return err!(Code::Internal, "ModuleContextData::new: target is not a module")
//         };
//         Ok(ModuleContextData {
//             context,
//             module_id,
//         })
//     }
// }

pub trait ClientConfig {
    fn service_supported(&self, service: &str) -> bool;
    fn method_supported(&self, method: &str) -> bool;
}

pub trait ModuleInit {
    fn new(config_bytes: &[u8], client_config: &dyn ClientConfig) -> Self;
}

pub trait Module<R=ServerRequestWrapperImpl>: ModuleServiceResolver<R> + ModuleInit {
    // type Config;

    fn describe<T: DescribeModule>(describe: &mut T) -> ModuleDescriptor;

    // fn describe(descriptor: &mut crate::types::cosmos::core::v1alpha1::bundle::ModuleInitDescriptor) -> zeropb::Result<()>;
    // fn route(&self, route_id: u64, ctx: &mut Context, req: *mut u8, res: *mut *mut u8) -> Code;
}

pub trait ModuleDyn<R> {
    fn new(&self, config_bytes: &[u8]) -> Box<dyn ModuleServiceResolver<R>>;
    fn describe(&self) -> ModuleDescriptor;
}

pub struct ModuleDescriptor {
    pub config_type_name: String,
    pub services: Vec<ServiceDescriptor>,
    pub clients: Vec<ClientDescriptor>
}

pub trait DescribeModule {
    // fn describe_service<T: Service>(&mut self);
    fn describe_client<T: Client>(&mut self);
}
