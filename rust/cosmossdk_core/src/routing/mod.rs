extern crate core;
use core::result::{Result::Err};
use crate::{Code, Context, err, Result};
use crate::Code::Unimplemented;
use crate::raw::{RawBox, RawBytes};
use core::convert::{From, Into};

// alternate designs
pub trait ServiceHandler {
    // fn route(&self, method_id: u64, ctx: &mut Context, req: *mut u8, res: *mut *mut u8) -> Code { Unimplemented }
    fn route_io(&self, method_id: u16, ctx: &mut Context, req: &[u8]) -> Result<RawBytes> { err!(Unimplemented) }
    fn route_i1(&self, method_id: u16, ctx: &mut Context, p1: &[u8]) -> Result<()> { err!(Unimplemented) }
    fn route_i2(&self, method_id: u16, ctx: &mut Context, p1: &[u8], p2: &[u8]) -> Result<()> { err!(Unimplemented) }
    // fn route_method(&self, method: &str, ctx: &mut Context, req: &[u8]) -> Result<RawBytes> { Err(Unimplemented.into()) }
    // fn route_service_method(&self, svc: &str, method: &str, ctx: &mut Context, req: &[u8]) -> Result<RawBytes> { Err(Unimplemented.into()) }
}

pub trait Server: ServiceHandler{
    fn describe() -> ServiceDescriptor;
}

// pub trait Server: ServiceHandler {
    // fn describe(descriptor: &mut crate::types::cosmos::core::v1alpha1::bundle::ModuleOutput) -> zeropb::Result<()>;
    // fn route_io(&self, method_id: u64, ctx: &mut Context, req: &[u8]) -> Result<RawBytes> { Err(Unimplemented.into()) }
    // fn route_i1(&self, method_id: u64, ctx: &mut Context, p1: &[u8]) -> Result<()> { Err(Unimplemented.into()) }
    // fn route_i2(&self, method_id: u64, ctx: &mut Context, p1: &[u8], p2: &[u8]) -> Result<()> { Err(Unimplemented.into()) }
// }

pub trait ClientRouter {
    // fn route_io(&self, method_id: u64, ctx: &mut Context, req: &[u8]) -> Result<RawBytes> { Err(Unimplemented.into()) }
    // fn route_i1(&self, method_id: u64, ctx: &mut Context, p1: &[u8]) -> Result<()> { Err(Unimplemented.into()) }
    // fn route_i2(&self, method_id: u64, ctx: &mut Context, p1: &[u8], p2: &[u8]) -> Result<()> { Err(Unimplemented.into()) }
}

pub struct ClientConnection<'a> {
    router: &'a dyn ClientRouter,
    route_id: u64,
}

pub trait Client<'a> {
    fn new(conn: ClientConnection<'a>) -> Self;
}

pub struct ServiceDescriptor {
    service_type: ServiceType,
    id: String,
}

pub enum ServiceType {
    ProtoService,
    ProtoServiceMethod,
    ProtoMessage,
    ProtoMessageBefore,
    ProtoMessageAfter,
    ProtoEventHook,
    Store,
}


