extern crate alloc;

use alloc::sync::Weak;

use crate::{err, error};
use crate::module::{Module, ModuleDyn};
use crate::routing::{CallData, Client, ClientConnection, ClientFactory, LocalRouteInfo, ModuleServiceResolver, RouteInfo, Router};

/// A router that only routes directly to services based on local routing info and
/// delegates all client calls to a remote client.
pub struct DirectRouter {
    client_router: Weak<dyn Router>,
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

pub struct DirectRouterBuilder {
    index: usize,
    client_router: Weak<dyn Router>,
    router: DirectRouter,
    module_configs: Vec<Vec<u8>>,
}

impl DirectRouterBuilder {
    pub fn new(client_router: Weak<dyn Router>) -> Self {
        DirectRouterBuilder {
            index: 0,
            client_router: client_router.clone(),
            router: DirectRouter {
                client_router,
                modules: Vec::new(),
            },
            module_configs: Vec::new(),
        }
    }

    pub fn add_module<T: Module + 'static>(&mut self, config_bytes: &[u8]) {
        let module = Box::new(T::new(config_bytes, self));
        self.router.modules.push(module);
        self.index += 1;
    }

    pub fn add_module_dyn(&mut self, m: &dyn ModuleDyn, config_bytes: &[u8]) {
        let module = m.new(config_bytes,
                           ClientConnection::new(self.client_router.clone(), RouteInfo::Empty),
        );
        self.router.modules.push(module);
        self.index += 1;
    }

    pub fn build(self) -> crate::Result<DirectRouter> {
        Ok(self.router)
    }
}

impl ClientFactory for DirectRouterBuilder {
    fn new<T: Client>(&self) -> T {
        T::new(ClientConnection::new(self.client_router.clone(), RouteInfo::Empty))
    }
}
