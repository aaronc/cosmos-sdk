use cosmossdk_core::mem::Ref;
use cosmossdk_core::{Context, ReadContext};
use cosmossdk_core::store::StoreClient;
use crate::key_codec::{encode_with_prefix, KeyCodec};
use crate::value_codec::ValueCodec;

pub struct Map<K, V> {
    _k: std::marker::PhantomData<K>,
    _v: std::marker::PhantomData<V>,

    store: StoreClient,
    name: String,
    prefix: Vec<u8>,
}

impl<K: KeyCodec, V: ValueCodec> Map<K, V> {
    pub fn new(store: StoreClient, prefix: &[u8], name: String /*TODO: , keys_names: K::Keys<'_>, values_names: &V::Keys<'_>*/) -> Self {
        Self {
            _k: std::marker::PhantomData,
            _v: std::marker::PhantomData,
            store,
            name,
            prefix: prefix.to_vec(),
        }
    }

    pub fn get<'a>(&self, ctx: &dyn ReadContext, key: K::Borrowed<'_>) -> cosmossdk_core::Result<V::AsRef<'a>> {
        let key_bytes = encode_with_prefix::<K>(&self.prefix, key)?;
        let value_bytes = self.store.get(ctx, &key_bytes)?;
        let (value, _left) = <V as ValueCodec>::decode(value_bytes.data)?;
        Ok(<V as ValueCodec>::as_ref(value, value_bytes))
    }

    pub fn set(&self, ctx: &dyn Context, key: K::Borrowed<'_>, value: V::Borrowed<'_>) -> cosmossdk_core::Result<()> {
        let key_bytes = encode_with_prefix::<K>(&self.prefix, key)?;
        let size_hint = V::size_hint(&value).unwrap_or(1024);
        let mut value_bytes = Vec::with_capacity(size_hint);
        V::encode(&mut value_bytes, value)?;
        self.store.set(ctx, &key_bytes, &value_bytes)
    }
}
