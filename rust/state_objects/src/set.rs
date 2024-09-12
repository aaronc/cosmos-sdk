use cosmos_context::Context;
use crate::Map;
use crate::map::{DeleteError, HasError, SetError};

#[derive(Default)]
pub struct Set<K, const Prefix: u8> {
    map: Map<K, (), Prefix>,
}

#[derive(Default)]
pub struct OrderedSet<K, const Prefix: u8> {
    set: OrderedSet<K, Prefix>,
}

pub trait SetTrait<K> {
    fn has(&self, ctx: &Context, key: &K) -> Result<bool, HasError>;
    fn set(&self, ctx: &mut Context, key: &K) -> Result<(), SetError>;
    fn delete(&self, ctx: &mut Context, key: &K) -> Result<(), DeleteError>;
}

pub trait OrderedSetTrait<K>: SetTrait<K> {
    // TODO prefix keys
    // fn iter(&self, ctx: &Context, start: &K, end: &K) -> Iter<K>;
    // fn reverse_iter(&self, ctx: &Context, start: &K, end: &K) -> Iter<K>;
}

pub struct Iter<K> {}
