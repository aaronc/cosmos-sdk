use cosmos_context::Context;

pub struct Map<K, V> {}

impl<K, V> Map<K, V> {
    pub fn get(&self, ctx: &Context, key: &K) -> Result<V, GetError> {
        Err(GetError::NotFound)
    }
    pub fn set(&self, ctx: &mut Context, key: &K, value: &V) -> Result<(), SetError> {
        Ok(())
    }
}

pub enum GetError {
    NotFound,
}

pub enum SetError {}
