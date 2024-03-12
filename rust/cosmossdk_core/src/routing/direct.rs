use crate::Module;
use crate::routing::{CallData, Client, ClientConnection, ClientFactory, ModuleServiceResolver, RouteInfo, Router};
use crate::routing::bundle::ModuleBundleVisitor;

/// A router that only routes directly to services based on local routing info and
/// delegates all client calls to a remote client.
pub struct DirectRouter<'a> {
    client_conn: ClientConnection<'a>,
    modules: Vec<Box<dyn ModuleServiceResolver>>
}

impl Router for DirectRouter<'_> {
    fn invoke(&self, call_data: &mut CallData) -> crate::Result<()> {
        todo!()
    }
}

impl <'a> ClientFactory<'a> for DirectRouter<'a> {
    fn new<T: Client<'a>>(&self) -> T {
        T::new(ClientConnection::new(self, RouteInfo::Empty))
    }
}

struct DirectRouterBuilder<'a> {
    index: usize,
    router: &'a mut DirectRouter<'a>,
    module_configs: Vec<Vec<u8>>
}

impl <'a> ModuleBundleVisitor for DirectRouterBuilder<'a> {
    fn visit_module<T: Module>(&mut self) {
        T::new(&self.module_configs[self.index], self.router);
        self.index += 1;
    }
}
