extern crate core;
extern crate alloc;
use core::ops::FnOnce;
use core::todo;
use super::{Result, Code, ReadContext, Context};
use core::result::{Result::{Err, Ok}};
use core::option::{Option};
use core::option::Option::{Some, None};
use crate::id::AgentId;
use crate::routing::{CallTarget, Client, ClientCallArgs, ClientConnection, ClientDescriptor, ClientDescriptorHelper};
use alloc::vec::Vec;
use crate::mem::{BytesRef, Ref};

#[cfg(feature="alloc")]
use crate::sync::{Completer, Completer1, PrepareContext};

pub struct StoreClient {
    conn: ClientConnection
}

pub trait StoreServer {
    fn get<Ctx: ReadContext>(&self, ctx: &Ctx, key: &[u8]) -> Result<Vec<u8>>;

    fn set<Ctx: Context>(&self, ctx: &Ctx, key: &[u8], value: &[u8]) -> Result<()>;

    fn delete<Ctx: Context>(&self, ctx: &mut Ctx, key: &[u8]) -> Result<()>;

    fn has<Ctx: ReadContext>(&self, ctx: &mut Ctx, key: &[u8]) -> Result<bool>;

    fn get_stale<Ctx: ReadContext>(&self, ctx: &mut Ctx, key: &[u8]) -> Result<Vec<u8>>;

    fn set_lazy<Ctx: Context, F: FnOnce(Option<&[u8]>) -> Option<Vec<u8>>>(&self, ctx: &mut Ctx, caller: &AgentId, key: &[u8], value_fn: F) -> Result<()>;

    #[cfg(feature="alloc")]
    fn prepare_get(&self, ctx: &PrepareContext, key: &[u8]) -> Result<Completer<Vec<u8>>> {
        Ok(alloc::boxed::Box::new(move |ctx| self.get(ctx, key)))
    }

    #[cfg(feature="alloc")]
    fn prepare_set(&self, ctx: &PrepareContext, key: &[u8]) -> Result<Completer1<Vec<u8>, ()>> {
        Ok(alloc::boxed::Box::new(move |ctx, value| self.set(ctx, key, value)))
    }

    #[cfg(feature="alloc")]
    fn prepare_delete(&self, ctx: &PrepareContext, key: &[u8]) -> Result<Completer<()>> {
        Ok(alloc::boxed::Box::new(move |ctx| self.delete(ctx, key)))
    }

    #[cfg(feature="alloc")]
    fn prepare_has(&self, ctx: &PrepareContext, key: &[u8]) -> Result<Completer<bool>> {
        Ok(alloc::boxed::Box::new(move |ctx| self.has(ctx, key)))
    }

    #[cfg(feature="alloc")]
    fn prepare_get_stale(&self, ctx: &PrepareContext, key: &[u8]) -> Result<Completer<Vec<u8>>> {
        Ok(alloc::boxed::Box::new(move |ctx| self.get_stale(ctx, key)))
    }

    #[cfg(feature="alloc")]
    fn prepare_set_lazy(&self, ctx: &PrepareContext, key: &[u8]) -> Result<Completer1<fn(&[u8]) -> Vec<u8>, ()>> {
        Ok(alloc::boxed::Box::new(move |ctx, value_fn| self.set_lazy(ctx, key, value_fn)))

    }
}

// impl Router for dyn Store {
//
// }

// impl Server for dyn Store {
//
// }

impl Client for StoreClient {
    fn describe(helper: &mut dyn ClientDescriptorHelper) -> ClientDescriptor {
        ClientDescriptor::StoreClient{ordered: false}
    }

    fn new(conn: ClientConnection) -> Self {
        todo!()
    }
}

impl StoreClient {
    pub fn get<Ctx: ReadContext>(&self, ctx: &Ctx, key: &[u8]) -> Result<Vec<u8>> {
        let mut call_args = ClientCallArgs::default();
        call_args.set_dynamic_route_target(CallTarget::StoreMethod("get".to_string()));
        call_args.set_in1(key);
        self.conn.invoke(ctx, &mut call_args)?;
        // TODO figure out how to pass BytesRef
        Ok(call_args.out1().to_vec())
    }

    pub fn set<Ctx: Context>(&self, ctx: &Ctx, key: &[u8], value: &[u8]) -> Result<()> {
        let mut call_args = ClientCallArgs::default();
        call_args.set_dynamic_route_target(CallTarget::StoreMethod("set".to_string()));
        call_args.set_in1(key);
        call_args.set_in2(value);
        self.conn.invoke(ctx, &mut call_args)
    }

    fn delete<Ctx: Context>(&self, ctx: &mut Ctx, key: &[u8]) -> Result<()> {
        // self.conn.route_i1(self.route_id & 0x3, ctx, key)
        todo!()
    }

    fn has<Ctx: ReadContext>(&self, ctx: &mut Ctx, key: &[u8]) -> Result<bool> {
        // match self.conn.route_io(self.route_id & 0x4, ctx, key) {
        //     Ok(_) => Ok(true),
        //     Err(e) => {
        //         if e.code == Code::NotFound {
        //             Ok(false)
        //         } else {
        //             Err(e)
        //         }
        //     }
        // }t
        todo!()
    }

    fn get_stale<Ctx: ReadContext>(&self, ctx: &mut Ctx, key: &[u8]) -> Result<Vec<u8>> {
        // self.conn.route_io(self.route_id & 0x5, ctx, key)
        todo!()
    }

    fn set_lazy<Ctx: Context>(&self, ctx: &mut Ctx, key: &[u8], value_fn: fn(&[u8]) -> Vec<u8>) -> Result<()> {
        // self.conn.route_i2(self.route_id & 0x6, ctx, key, value_fn(key))
        todo!()
    }
}
