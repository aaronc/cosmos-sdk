extern crate core;

use crate::{Context, Result};

// alternate designs
pub trait ServiceHandler {
    fn invoke(&self, method_id: u16, ctx: &mut Context, call_data: &mut CallArgs) -> Result<()>;
}

pub trait Service: ServiceHandler {
    fn describe(helper: &mut dyn ServiceDescriptorHelper) -> ServiceDescriptor;
}

pub trait ServiceDescriptorHelper {
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

pub trait ClientFactory {
    fn new<T: Client>(&self) -> T;
}

pub struct ClientConnection<'a> {
    router: &'a dyn ClientRouter,
    route_token: u128,
}

pub trait Client<'a> {
    fn describe(helper: &mut dyn ClientDescriptorHelper) -> ClientDescriptor;
    fn new(conn: ClientConnection<'a>) -> Self;
}

pub enum ClientDescriptor {
    DynamicProtoClient
}

pub trait ClientDescriptorHelper {}

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

#[repr(C)]
struct CallData {
    context: Context,
    route_info: RouteInfo,
    data: CallArgs,
}

#[repr(C)]
pub struct CallArgs {
    in1: BytesPtr,
    in2: BytesPtr,
    out1: BytesPtr,
    out2: BytesPtr,
}

impl CallArgs {
    pub fn in1(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.in1.ptr, self.in1.len) }
    }

    pub fn in2(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.in2.ptr, self.in2.len) }
    }

    pub fn set_out1(&mut self, mut bytes: Vec<u8>) {
        unsafe {
            let len = bytes.len();
            let ptr = bytes.as_mut_ptr();
            self.out1 = BytesPtr { len, ptr };
        }
    }

    pub fn set_out2(&mut self, mut bytes: Vec<u8>) {
        unsafe {
            let len = bytes.len();
            let ptr = bytes.as_mut_ptr();
            self.out2 = BytesPtr { len, ptr };
        }
    }
}

#[repr(C)]
struct RouteInfo {
    module_index: u32,
    service_index: u16,
    method_index: u16,
}

#[repr(C)]
struct BytesPtr {
    len: usize,
    ptr: *mut u8,
}
