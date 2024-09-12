use cosmos_context::Context;

pub struct Map<K, V, const Prefix: u8 = 0> {
    prefix: arrayvec::ArrayVec<u8, 4>,
}

impl<K, V, const Prefix: u8> Default for Map<K, V, Prefix> {
    fn default() -> Self {
        let mut prefix = arrayvec::ArrayVec::new();
        prefix.push(Prefix);
        Self { prefix }
    }
}

impl<K, V, const Prefix: u8> Map<K, V, Prefix> {
    pub unsafe fn get_prefix(&self) -> &[u8] {
        self.prefix.as_slice()
    }

    pub unsafe fn set_prefix(&mut self, prefix: &[u8]) {
        todo!()
    }
}

impl <K, V, const Prefix: u8> MapTrait<K, V> for Map<K, V, Prefix> {
    fn has(&self, ctx: &Context, key: &K) -> Result<bool, HasError> {
        todo!()
    }

    fn get(&self, ctx: &Context, key: &K) -> Result<V, GetError> {
        todo!()
    }

    fn set(&self, ctx: &mut Context, key: &K, value: &V) -> Result<(), SetError> {
        todo!()
    }
}

#[derive(Default)]
pub struct OrderedMap<K, V, const Prefix: u8 = 0> {
    map: Map<K, V, Prefix>,
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

impl<K, V> Iterator for Iter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}