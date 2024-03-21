pub mod direct_router;
pub mod app_router;

extern crate alloc;
extern crate core;

use core::any::Any;
use crate::{AgentId, Code, error, Result, Context, ReadContext, ModuleId};
use crate::id::Address;
use crate::module::{ModuleContext, ModuleReadContext};

// alternate designs
pub trait ServiceHandler {
    fn invoke(&self, method_id: u32, ctx: &ContextData, call_data: &mut CallArgs) -> Result<()>;
}

pub trait Service: ServiceHandler {
    fn describe(helper: &mut dyn ServiceDescriptorHelper) -> ServiceDescriptor;
}

pub trait ServiceDescriptorHelper {}

// pub trait Server: ServiceHandler {
// fn describe(descriptor: &mut crate::types::cosmos::core::v1alpha1::bundle::ModuleOutput) -> zeropb::Result<()>;
// fn route_io(&self, method_id: u64, ctx: &mut Context, req: &[u8]) -> Result<RawBytes> { Err(Unimplemented.into()) }
// fn route_i1(&self, method_id: u64, ctx: &mut Context, p1: &[u8]) -> Result<()> { Err(Unimplemented.into()) }
// fn route_i2(&self, method_id: u64, ctx: &mut Context, p1: &[u8], p2: &[u8]) -> Result<()> { Err(Unimplemented.into()) }
// }

pub trait Router: Any {
    fn invoke(&self, call_data: &mut CallData) -> Result<()>;
}

pub trait ClientFactory {
    fn new<T: Client>(&self) -> T;
}

pub struct ClientConnection {
    router: alloc::sync::Weak<dyn Router>,
    default_route_info: RouteInfo,
}

impl ClientConnection {
    pub fn new(router: alloc::sync::Weak<dyn Router>, default_route_info: RouteInfo) -> Self {
        ClientConnection {
            router,
            default_route_info,
        }
    }

    pub fn invoke<Ctx: ReadContext>(&self, ctx: &Ctx, args: &mut ClientCallArgs) -> Result<()> {
        args.0.context.id = ctx.id();
        args.0.context.source = ctx.self_id().clone();
        let router = &self.router.upgrade().ok_or(
            error!(Code::Internal, "Router has been dropped")
        )?;
        router.invoke(&mut args.0)
    }
}

pub trait Client {
    fn describe(helper: &mut dyn ClientDescriptorHelper) -> ClientDescriptor;
}

#[derive(Clone)]
pub enum CallTarget {
    ProtoMessage(String),
    ProtoMethod(String),
    StoreMethod(String),
}

pub enum ClientDescriptor {
    ServiceClient(String),
    ConcreteClient(CallTarget),
    DynamicProtoClient,
    StoreClient{ordered: bool}
}

pub trait ClientDescriptorHelper {}

// pub struct ServiceDescriptor {
//     service_type: ServiceType,
//     id: String,
// }
//
// pub enum ServiceType {
//     ProtoService,
//     ProtoServiceMethod,
//     ProtoMessage,
//     ProtoMessageBefore,
//     ProtoMessageAfter,
//     ProtoEventHook,
//     Store,
// }

pub enum ServiceDescriptor {
    ProtoService(ProtoSericeDescriptor)
}

pub struct ProtoSericeDescriptor {
    name: String,
    encoding: Encoding,
    methods: Vec<ProtoMethodDescriptor>
}

pub struct ProtoMethodDescriptor {
    name: String,
    input_type: String,
}

#[derive(Clone, Debug)]
pub enum Encoding {
    Other,
    ProtoBinary,
    ZeroPB
}


#[repr(C)]
struct CallData {
    context: ContextData,
    data: CallArgs,
    route_info: RouteInfo,
}

pub struct ClientCallArgs(CallData);

impl Default for ClientCallArgs {
    fn default() -> Self {
        unsafe {
            let ptr = alloc::alloc::alloc(alloc::alloc::Layout::from_size_align(
                core::mem::size_of::<CallData>(),
                core::mem::align_of::<CallData>()
            ).unwrap());
            ClientCallArgs(core::ptr::read(ptr as *const CallData))
        }
    }
}

impl ClientCallArgs {
    pub fn set_target_address(&mut self, address: Address) {
        self.0.context.target = AgentId::Account(address);
    }

    pub fn set_dynamic_route_target(&mut self, target: CallTarget) {
        self.0.route_info = RouteInfo::ClientTarget(target);
    }

    pub fn set_in1<'a>(&'a mut self, mut bytes: &'a [u8]) {
        unsafe {
            let len = bytes.len();
            let ptr = bytes.as_ptr() as *mut u8;
            self.0.data.in1 = BytesPtr { len, ptr };
        }
    }

    pub fn set_in2<'a>(&'a mut self, mut bytes: &'a [u8]) {
        unsafe {
            let len = bytes.len();
            let ptr = bytes.as_ptr() as *mut u8;
            self.0.data.in2 = BytesPtr { len, ptr };
        }
    }

    pub fn out1(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.0.data.out1.ptr, self.0.data.out1.len) }
    }

    pub fn out2(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.0.data.out2.ptr, self.0.data.out2.len) }
    }
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
#[derive(Clone)]
enum RouteInfo {
    Empty,
    Local(LocalRouteInfo),
    ClientToken(u128),
    ClientTarget(CallTarget),
}

#[repr(C)]
#[derive(Clone)]
struct LocalRouteInfo {
    module_index: rend::u32_le,
    service_index: rend::u32_le,
    method_index: rend::u32_le,
}

#[repr(C)]
struct BytesPtr {
    len: usize, // TODO rend::u32_le
    ptr: *mut u8, // TODO rend::u64_le
}

pub trait ModuleServiceResolver {
    fn resolve_service_handler(&self, index: u32) -> Option<&dyn ServiceHandler>;
}

#[repr(C)]
pub struct ContextData {
    pub(crate) id: u64,
    pub(crate) source: AgentId,
    pub(crate) target: AgentId,
    _padding: [u8; 508], // extra space for future use and makes context 1024 bytes
}

impl ReadContext for ContextData {
    fn id(&self) -> u64 {
        self.id
    }

    fn self_id(&self) -> &AgentId {
        &self.target
    }
}

impl Context for ContextData {
    fn caller_id(&self) -> &AgentId {
        &self.source
    }
}
