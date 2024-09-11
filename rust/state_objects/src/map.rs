use cosmos_context::Context;

#[derive(Default)]
pub struct Map<const Prefix: u8, K, V>;

#[derive(Default)]
pub struct OrderedMap<const Prefix: u8, K, V> {
    map: Map<Prefix, K, V>,
}

trait MapTrait<K, V> {
    fn has(&self, ctx: &Context, key: &K) -> Result<bool, HasError>;
    fn get(&self, ctx: &Context, key: &K) -> Result<V, GetError>;
    fn set(&self, ctx: &mut Context, key: &K, value: &V) -> Result<(), SetError>;
}

trait OrderedMapTrait<K, V>: MapTrait<K, V> {
    // TODO prefix keys
    // fn iter(&self, ctx: &Context, start: &K, end: &K) -> Iter<K, V>;
    // fn reverse_iter(&self, ctx: &Context, start: &K, end: &K) -> Iter<K, V>;
}

pub struct Iter<K, V> {}

pub enum HasError {}

pub enum GetError {
    NotFound,
}

pub enum SetError {}

pub enum DeleteError {}

impl <K, V> Iterator for Iter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}