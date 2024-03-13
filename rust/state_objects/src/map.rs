use cosmossdk_core::mem::Ref;
use cosmossdk_core::{Context, ReadContext};
use cosmossdk_core::store::StoreClient;
use crate::buffer::BytesReader;
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
        let reader = BytesReader::new(value_bytes.data);
        let value = <V as ValueCodec>::decode(&reader)?;
        Ok(<V as ValueCodec>::as_ref(value, value_bytes))
    }

    pub fn set(&self, ctx: &dyn Context, key: K::Borrowed<'_>, value: &V::Borrowed<'_>) -> cosmossdk_core::Result<()> {
        todo!()
    }
}
