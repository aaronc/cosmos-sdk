use cosmos_context::{Context, Response};
use crate::Map;
use crate::map::{GetError, SetError};

#[derive(Default)]
pub struct Item<V, const Prefix: u8 = 0> {
    map: Map<(), V, Prefix>,
}

pub trait ItemTrait<V: Default> {
    fn get(&self, ctx: &Context) -> Result<V, GetError>;
    fn set(&self, ctx: &mut Context, value: &V) -> Result<(), SetError>;
}

impl <V, const Prefix: u8> ItemTrait<V> for Item<V, Prefix> {
    fn get(&self, ctx: &Context) -> Response<V, GetError> {
        // self.map.get(ctx, &())
        todo!()
    }

    fn set(&self, ctx: &mut Context, value: &V) -> Response<(), SetError> {
        // self.map.set(ctx, &(), value)
        todo!()
    }
}