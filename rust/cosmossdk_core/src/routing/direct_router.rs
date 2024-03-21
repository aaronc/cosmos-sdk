extern crate alloc;

use alloc::sync::Weak;

use crate::{err, error};
use crate::module::{Module, ModuleDyn};
use crate::routing::{CallData, Client, LocalRouteInfo, ModuleServiceResolver, RouteInfo, Router};

/// A router that only routes directly to services based on local routing info and
/// delegates all client calls to a remote client.
#[derive(Default)]
pub struct DirectRouter {
    modules: Vec<Box<dyn ModuleServiceResolver>>,
}

impl Router for DirectRouter {
    fn invoke(&self, call_data: &mut CallData) -> crate::Result<()> {
        match call_data.route_info {
            RouteInfo::Local(LocalRouteInfo { module_index, service_index, method_index }) => {
                let module_index: usize = module_index.value() as usize;
                if module_index >= self.modules.len() {
                    return err!(crate::Code::Internal, "Module index out of range");
                }
                let module = &self.modules[module_index];
                let service = module.resolve_service_handler(service_index.value()).ok_or(
                    error!(crate::Code::Internal, "Service index out of range")
                )?;
                service.invoke(method_index.value(), &mut call_data.context, &mut call_data.data)
            }
            _ => return err!(crate::Code::Unimplemented, "Direct router does not support route translation")
        }
    }
}

impl DirectRouter {
    pub fn add_module<T: Module + 'static>(&mut self, config_bytes: &[u8]) {
        self.modules.push(T::new(config_bytes))
    }

    pub fn add_module_dyn(&mut self, m: &dyn ModuleDyn, config_bytes: &[u8]) {
        self.modules.push(m.new(config_bytes))
    }
}