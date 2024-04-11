use std::borrow::Cow;
use cosmos_core_api::{Context, ReadContext};

pub trait Store {
    fn has<Ctx: ReadContext<'_>>(&self, ctx: &Ctx, key: &[u8]) -> cosmos_result::Result<bool>;

    fn get<'a, Ctx: ReadContext<'a>>(&self, ctx: &'a Ctx, key: &[u8]) -> cosmos_result::Result<Cow<'a, [u8]>>;

    fn set<Ctx: Context<'_>>(&self, ctx: &Ctx, key: &[u8], value: &[u8]) -> cosmos_result::Result<()>;

    fn delete<Ctx: Context<'_>>(&self, ctx: &Ctx, key: &[u8]) -> cosmos_result::Result<()>;
}

pub struct StoreClient;

impl Store for StoreClient {
    fn has<Ctx: ReadContext<'_>>(&self, ctx: &Ctx, key: &[u8]) -> cosmos_result::Result<bool> {
        todo!()
    }

    fn get<'a, Ctx: ReadContext<'a>>(&self, ctx: &'a Ctx, key: &[u8]) -> cosmos_result::Result<Cow<'a, [u8]>> {
        let mut req = ctx.new_request();
        req.set_target_method("store/get");
        req.in_params()[0].set_bytes(key);
        ctx.invoke(&mut req)?;
        Ok(req.out_params()[0].bytes().to_vec())
    }

    fn set<Ctx: Context<'_>>(&self, ctx: &Ctx, key: &[u8], value: &[u8]) -> cosmos_result::Result<()> {
        todo!()
    }

    fn delete<Ctx: Context>(&self, ctx: &Ctx, key: &[u8]) -> cosmos_result::Result<()> {
        unimplemented!()
    }
}