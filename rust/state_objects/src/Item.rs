use cosmos_context::Context;
use crate::Map;
use crate::map::{GetError, SetError};

#[derive(Default)]
pub struct Item<const Prefix: u8, V> {
    map: Map<Prefix, (), V>,
}

pub trait ItemTrait<V: Default> {
    fn get(&self, ctx: &Context) -> Result<V, GetError>;
    fn set(&self, ctx: &mut Context, value: &V) -> Result<(), SetError>;
}
