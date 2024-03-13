extern crate alloc;
use alloc::sync::Arc;
use crate::bundle::ModuleBundleVisitor;
use crate::module::Module;
use crate::routing::{CallData, Client, ClientConnection, ClientFactory, ModuleServiceResolver, RouteInfo, Router};

/// A router that only routes directly to services based on local routing info and
/// delegates all client calls to a remote client.
pub struct DirectRouter {
    client_conn: ClientConnection,
    modules: Vec<Box<dyn ModuleServiceResolver>>
}

impl Router for DirectRouter {
    fn invoke(&self, call_data: &mut CallData) -> crate::Result<()> {
        todo!()
    }
}

impl ClientFactory for DirectRouterBuilder {
    fn new<T: Client>(&self) -> T {
        T::new(ClientConnection::new(Arc::downgrade(&self.router_arc), RouteInfo::Empty))
    }
}

struct DirectRouterBuilder {
    index: usize,
    router: DirectRouter,
    router_arc: Arc<dyn Router>,
    module_configs: Vec<Vec<u8>>
}

impl ModuleBundleVisitor for DirectRouterBuilder {
    fn visit_module<T: Module>(&mut self) {
        T::new(&self.module_configs[self.index], self);
        self.index += 1;
    }
}
