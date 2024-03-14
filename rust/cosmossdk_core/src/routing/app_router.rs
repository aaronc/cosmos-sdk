extern crate alloc;

use alloc::sync::Arc;
use alloc::collections::BTreeMap;
use std::ptr::null_mut;
use crate::bundle::{ModuleBundle, ModuleBundleVisitor};
use crate::module::{DescribeModule, Module};
use crate::routing::direct_router::DirectRouter;
use crate::routing::{CallData, Client, Encoding, LocalRouteInfo, RouteInfo, Router, Service, ServiceDescriptor, ServiceDescriptorHelper, ServiceHandler};

pub struct AppRouter {
    direct_router: *mut Box<DirectRouter>,
    route_translation_table: BTreeMap<String, ResolvedRoute>,
}

enum ResolvedRoute {
    ModuleMessage { pre: Vec<ResolvedRouteInfo>, handle: Option<ResolvedRouteInfo>, post: Vec<ResolvedRouteInfo> },
    Query(ResolvedRouteAddress),
}

struct ResolvedRouteInfo {
    address: ResolvedRouteAddress,
    encoding: Encoding,
}

enum ResolvedRouteAddress {
    Local(LocalRouteInfo),
    Remote { loader_id: u32, bundle_id: u32, local: LocalRouteInfo },
}

impl AppRouter {
    pub fn build<B: ModuleBundle>(module_configs: Vec<Vec<u8>>) -> crate::Result<Arc<dyn Router>> {
        let mut route_table_builder = RouteTableBuilder {
            table: Default::default(),
            module_idx: 0,
            service_idx: 0,
        };
        B::visit(&mut route_table_builder)?;
        let mut router = Self {
            direct_router: null_mut(),
            route_translation_table: route_table_builder.table,
        };
        let mut direct_router_ptr = unsafe { router.direct_router };
        let self_arc: Arc<dyn Router> = Arc::new(router);
        let direct_router = Box::new(DirectRouter::build::<B>(module_configs, Arc::downgrade(&self_arc))?);
        unsafe {
            *direct_router_ptr = direct_router;
        }
        Ok(self_arc)
    }
}


impl Router for AppRouter {
    fn invoke(&self, call_data: &mut CallData) -> crate::Result<()> {
        match call_data.route_info {
            RouteInfo::Empty => {}
            RouteInfo::Local(_) => {}
            RouteInfo::ClientToken(_) => {}
            RouteInfo::ClientTarget(_) => {}
        }
        let direct_router = unsafe { &*self.direct_router };
        direct_router.invoke(call_data)
    }
}

struct RouteTableBuilder {
    table: BTreeMap<String, ResolvedRoute>,
    module_idx: u32,
    service_idx: u32,
}

impl ModuleBundleVisitor for RouteTableBuilder {
    fn visit_module<T: Module + 'static>(&mut self) -> crate::Result<()> {
        let _descriptor = T::describe(self);
        self.module_idx += 1;
        self.service_idx = 0;
        Ok(())
    }
}

impl DescribeModule for RouteTableBuilder {
    fn describe_service<T: Service>(&mut self) {
        self.service_idx += 1;
        match T::describe(self) {
            ServiceDescriptor::ProtoService(svc) => {
                let method_idx = 0;
                for method in svc.methods {
                    // TODO check for existing route
                    let route = ResolvedRoute::ModuleMessage {
                        pre: vec![],
                        handle: Some(ResolvedRouteInfo {
                            address: ResolvedRouteAddress::Local(LocalRouteInfo {
                                module_index: self.module_idx.into(),
                                service_index: self.service_idx.into(),
                                method_index: method_idx.into(),
                            }),
                            encoding: svc.encoding.clone(),
                        }),
                        post: vec![],
                    };
                    self.table.insert(method.name, route);
                }
            }
        }
    }

    fn describe_client<T: Client>(&mut self) {}
}

impl ServiceDescriptorHelper for RouteTableBuilder {}