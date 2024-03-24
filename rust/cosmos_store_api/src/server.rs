extern crate alloc;

use alloc::borrow::Cow;
use cosmos_context_api::{Context, ReadContext};
use cosmos_result::{Result};

pub trait StoreServer<Ctx> {
    fn has(&self, ctx: &Ctx, key: &[u8]) -> Result<bool>;

    fn get<'a>(&self, ctx: &'a Ctx, key: &[u8]) -> Result<Cow<'a, [u8]>>;

    fn set(&self, ctx: &Ctx, key: &[u8], value: &[u8]) -> Result<()>;

    fn delete(&self, ctx: &Ctx, key: &[u8]) -> Result<()>;

    // fn get_stale(&self, ctx: &Ctx, key: &[u8]) -> Result<Cow<[u8]>>;

    // fn set_lazy<F: FnOnce(Option<&[u8]>) -> Option<Vec<u8>>>(&self, ctx: &mut Ctx, key: &[u8], value_fn: F) -> Result<()>;

    // #[cfg(feature="alloc")]
    // fn prepare_get(&self, ctx: &PrepareContext, key: &[u8]) -> Result<Completer<Vec<u8>>> {
    //     Ok(alloc::boxed::Box::new(move |ctx| self.get(ctx, key)))
    // }
    //
    // #[cfg(feature="alloc")]
    // fn prepare_set(&self, ctx: &PrepareContext, key: &[u8]) -> Result<Completer1<Vec<u8>, ()>> {
    //     Ok(alloc::boxed::Box::new(move |ctx, value| self.set(ctx, key, value)))
    // }
    //
    // #[cfg(feature="alloc")]
    // fn prepare_delete(&self, ctx: &PrepareContext, key: &[u8]) -> Result<Completer<()>> {
    //     Ok(alloc::boxed::Box::new(move |ctx| self.delete(ctx, key)))
    // }
    //
    // #[cfg(feature="alloc")]
    // fn prepare_has(&self, ctx: &PrepareContext, key: &[u8]) -> Result<Completer<bool>> {
    //     Ok(alloc::boxed::Box::new(move |ctx| self.has(ctx, key)))
    // }
    //
    // #[cfg(feature="alloc")]
    // fn prepare_get_stale(&self, ctx: &PrepareContext, key: &[u8]) -> Result<Completer<Vec<u8>>> {
    //     Ok(alloc::boxed::Box::new(move |ctx| self.get_stale(ctx, key)))
    // }
    //
    // #[cfg(feature="alloc")]
    // fn prepare_set_lazy(&self, ctx: &PrepareContext, key: &[u8]) -> Result<Completer1<fn(&[u8]) -> Vec<u8>, ()>> {
    //     Ok(alloc::boxed::Box::new(move |ctx, value_fn| self.set_lazy(ctx, key, value_fn)))
    //
    // }
}
