extern crate alloc;

use alloc::sync::Weak;

use crate::{err, error, ServerRequestWrapper};
use crate::module::{Module, ModuleDyn};
use crate::routing::{CallData, Client, LocalRouteInfo, ModuleServiceResolver, RouteInfo, Router};

/// A router that only routes directly to services based on local routing info and
/// delegates all client calls to a remote client.
#[derive(Default)]
pub struct DirectRouter<ServerReq> {
    modules: Vec<Box<dyn ModuleServiceResolver<ServerReq>>>,
}

impl<R: ServerRequestWrapper> DirectRouter<R> {
    pub fn invoke(&self, route_info: &LocalRouteInfo, call_data: &mut R::R<'_>) -> crate::Result<()> {
        let LocalRouteInfo { module_index, service_index, method_index } = route_info;
        let module_index: usize = module_index.value() as usize;
        if module_index >= self.modules.len() {
            return err!(crate::Code::Internal, "Module index out of range");
        }
        let module = &self.modules[module_index];
        let service = module.resolve_service_handler(service_index.value()).ok_or(
            error!(crate::Code::Internal, "Service index out of range")
        )?;
        service.invoke(method_index.value(), call_data)
    }

    pub fn add_module<T: Module<R> + 'static>(&mut self, config_bytes: &[u8]) {
        // self.modules.push(T::new(config_bytes))
        todo!()
    }

    pub fn add_module_dyn(&mut self, m: &dyn ModuleDyn<R>, config_bytes: &[u8]) {
        self.modules.push(m.new(config_bytes))
    }
}