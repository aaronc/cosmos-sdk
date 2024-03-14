extern crate alloc;
use alloc::sync::{Arc, Weak};
use core::any::Any;
use crate::bundle::{ModuleBundle, ModuleBundleVisitor};
use crate::{err, error};
use crate::module::Module;
use crate::routing::{CallData, Client, ClientConnection, ClientFactory, LocalRouteInfo, ModuleServiceResolver, RouteInfo, Router};

/// A router that only routes directly to services based on local routing info and
/// delegates all client calls to a remote client.
pub struct DirectRouter {
    client_router: Weak<dyn Router>,
    modules: Vec<Box<dyn ModuleServiceResolver>>
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
    pub fn build<B: ModuleBundle>(module_configs: Vec<Vec<u8>>, client_router: Weak<dyn Router>) -> crate::Result<DirectRouter> {
        let mut builder = DirectRouterBuilder {
            index: 0,
            client_router: client_router.clone(),
            router: DirectRouter {
                client_router: client_router,
                modules: Vec::with_capacity(module_configs.len())
            },
            module_configs,
        };
        B::visit(&mut builder)?;
        Ok(builder.router)
    }
}

struct DirectRouterBuilder {
    index: usize,
    client_router: Weak<dyn Router>,
    router: DirectRouter,
    module_configs: Vec<Vec<u8>>
}

impl ClientFactory for DirectRouterBuilder {
    fn new<T: Client>(&self) -> T {
        T::new(ClientConnection::new(self.client_router.clone(), RouteInfo::Empty))
    }
}

impl ModuleBundleVisitor for DirectRouterBuilder {
    fn visit_module<T: Module + 'static>(&mut self) -> crate::Result<()> {
        let module = Box::new(T::new(&self.module_configs[self.index], self));
        self.router.modules.push(module);
        self.index += 1;
        Ok(())
    }
}
