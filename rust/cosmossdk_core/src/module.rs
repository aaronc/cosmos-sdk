use crate::{Code, Context};
use crate::routing::{ClientFactory, ServiceHandler};

pub trait Module {
    type Config;

    // fn describe(descriptor: &mut crate::types::cosmos::core::v1alpha1::bundle::ModuleInitDescriptor) -> zeropb::Result<()>;
    // fn route(&self, route_id: u64, ctx: &mut Context, req: *mut u8, res: *mut *mut u8) -> Code;
    fn describe<T: DescribeModule>(describe: &mut T);
    fn new<F: ClientFactory>(config: Self::Config, client_factory: &F) -> Self;
    fn resolve_service_handler(&self, index: u16) -> &dyn ServiceHandler;
}

pub trait DescribeModule {
    fn describe_service<T:ServiceHandler>();
}
