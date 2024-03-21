use crate::{Address, AgentId, ReadContext};
use crate::routing::Router;

struct Context<'a> {
    router: &'a dyn Router,
    data: &'a ContextData,
}

impl <'a> ReadContext for Context<'a> {
    type R = Request;

    fn id(&self) -> u64 {
        self.data.id
    }

    fn self_id(&self) -> &AgentId {
        &self.data.source
    }

    fn new_request(&self) -> Self::R {
        let mut req = Request::default();
        req.context.id = self.data.id;
        req.context.source = self.data.target.clone();
        req
    }

    fn invoke(&self, req: &mut Self::R) -> crate::Result<()> {
        todo!()
    }
}

impl <'a> crate::Context for Context<'a> {
    fn caller_id(&self) -> &AgentId {
        &self.data.source
    }
}

#[repr(C)]
#[derive(Default)]
struct ContextData {
    id: u64,
    source: AgentId,
    target: AgentId,
    _padding: [u8; 502], // extra space for future use and makes context 1024 bytes
}


#[repr(C)]
#[derive(Default)]
struct Request {
    method: Address,
    context: ContextData,
    in_params: [Param; 2],
    out_params: [Param; 2],
}

impl crate::ClientRequest for Request {
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

struct ServerRequest<'a> {
    router: &'a dyn Router,
    request: &'a mut Request,
}

impl <'a> crate::ServerRequest for ServerRequest<'a> {
    type Ctx = Context<'a>;
    type P = Param;

    fn context(&self) -> &Self::Ctx {
        &Context {
            router: self.router,
            data: &self.request.context,
        }
    }

    fn in_params(&mut self) -> &[Self::P; 2] {
        &self.request.in_params
    }

    fn out_params(&self) -> &mut [Self::P; 2] {
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
            let ptr = bytes.as_ptr() as *const u8 as u64;
            self.bytes.ptr = ptr;
            self.bytes.size = bytes.len() as u64;
        }
    }
}