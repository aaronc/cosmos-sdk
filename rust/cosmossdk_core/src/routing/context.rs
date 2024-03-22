use crate::{Address, AgentId, Code, err, ModuleId, ReadContext};
use crate::module::ModuleReadContext;
use crate::routing::Router;

pub struct ContextImpl<'a> {
    router: &'a dyn Router<RequestImpl>,
    data: &'a ContextData,
}

impl<'a> ReadContext for ContextImpl<'a> {
    type R = RequestImpl;

    fn id(&self) -> u64 {
        self.data.id
    }

    fn self_id(&self) -> &AgentId {
        &self.data.source
    }

    fn new_request(&self) -> Self::R {
        let mut req = RequestImpl::default();
        req.context.id = self.data.id;
        req.context.source = self.data.target.clone();
        req
    }

    fn invoke(&self, req: &mut Self::R) -> crate::Result<()> {
        self.router.invoke(req)
    }
}

impl<'a> crate::Context for ContextImpl<'a> {
    fn caller_id(&self) -> &AgentId {
        &self.data.source
    }
}

#[repr(C)]
struct ContextData {
    pub(crate) id: u64,
    pub(crate) source: AgentId,
    pub(crate) target: AgentId,
    _padding: [u8; 502], // extra space for future use and makes context 1024 bytes
}

impl Default for ContextData {
    fn default() -> Self {
        Self {
            id: 0,
            source: Default::default(),
            target: Default::default(),
            _padding: [0; 502],
        }
    }
}


#[repr(C)]
#[derive(Default)]
pub struct RequestImpl {
    pub(crate) method: Address,
    pub(crate) context: ContextData,
    pub(crate) in_params: [Param; 2],
    pub(crate) out_params: [Param; 2],
}

impl crate::ClientRequest for RequestImpl {
    type P = Param;

    fn set_target_method(&mut self, route: &str) -> crate::Result<()> {
        let addr = Address::new(route.as_bytes())?;
        self.method = addr;
        Ok(())
    }

    fn set_target_account(&mut self, account: Address) {
        self.context.target = AgentId::Account(account)
    }

    fn in_params(&mut self) -> &mut [Self::P; 2] {
        &mut self.in_params
    }

    fn out_params(&self) -> &[Self::P; 2] {
        &self.out_params
    }
}

pub struct ServerRequestImpl<'a> {
    pub(crate) request: &'a mut RequestImpl,
    pub(crate) context: &'a ContextImpl<'a>,
}

pub struct ServerRequestWrapperImpl;

impl crate::ServerRequestWrapper for ServerRequestWrapperImpl {
    type R<'a> = ServerRequestImpl<'a>;
}

impl<'a> crate::ServerRequest<'a> for ServerRequestImpl<'a> {
    type Ctx = ContextImpl<'a>;
    type P = Param;

    fn context(&self) -> Self::Ctx {
        &self.context
    }

    fn in_params(&self) -> &[Self::P; 2] {
        &self.request.in_params
    }

    fn out_params(&mut self) -> &mut [Self::P; 2] {
        &mut self.request.out_params
    }
}

#[repr(C)]
union Param {
    bytes: BytesParam,
    i: u128,
}

impl Default for Param {
    fn default() -> Self {
        Param { i: 0 }
    }
}

#[derive(Default, Copy, Clone)]
struct BytesParam {
    ptr: u64,
    size: u64,
}

impl crate::Param for Param {
    fn bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(self.bytes.ptr as *const u8, self.bytes.size as usize)
        }
    }

    fn set_bytes(&mut self, bytes: &[u8]) {
        unsafe {
            let ptr = bytes.as_ptr() as u64;
            self.bytes.ptr = ptr;
            self.bytes.size = bytes.len() as u64;
        }
    }
}

pub struct ModuleContextImpl<'a> {
    context: &'a ContextImpl<'a>,
    module_id: &'a ModuleId,
}

impl<'a> ModuleContextImpl<'a> {
    pub fn new(context: &'a ContextImpl) -> crate::Result<Self> {
        let AgentId::Module(module_id) = &context.target() else {
            return err!(Code::Internal, "ModuleContextData::new: target is not a module");
        };
        Ok(Self {
            context,
            module_id,
        })
    }
}

impl<'a> crate::Context for ModuleContextImpl<'a> {
    fn caller_id(&self) -> &AgentId {
        self.context.caller_id()
    }
}

impl<'a> ReadContext for ModuleContextImpl<'a> {
    type R = RequestImpl;

    fn id(&self) -> u64 {
        self.context.id()
    }

    fn self_id(&self) -> &AgentId {
        self.context.self_id()
    }

    fn new_request(&self) -> Self::R {
        self.context.new_request()
    }

    fn invoke(&self, req: &mut Self::R) -> crate::Result<()> {
        self.context.invoke(req)
    }
}

impl<'a> ModuleReadContext for ModuleContextImpl<'a> {
    fn module_id(&self) -> &ModuleId {
        self.context.module_id()
    }
}

impl<'a> crate::module::ModuleContext for ModuleContextImpl<'a> {}
