use cosmossdk_core::mem::Ref;
use cosmossdk_core::Result;
use crate::borrow::Ownable;
use crate::buffer::{Writer};

mod tuple;
mod bytes;
mod uint;
mod int;
mod bool;
mod str;

pub trait KeyCodec: Ownable {
    fn encode<B: Writer>(buf: &mut B, key: Self::T::Borrowed<'_>) -> Result<()>;

    fn decode<'a>(buf: &'a [u8]) -> Result<(Self::T::Borrowed<'a>, usize)>;

    fn size_hint(key: Self::Borrowed<'_>) -> usize;
    fn to_dynamic(key: Self::Borrowed<'_>) -> crate::dynamic::DynamicValue;

    fn from_value(value: crate::dynamic::DynamicValue) -> Result<Self::Borrowed<'static>>;
}

pub trait PrefixKey<Key> {}

impl<Key> PrefixKey<Key> for Key {}

pub fn encode_with_prefix<K: KeyCodec>(prefix: &[u8], key: K::Borrowed<'_>) -> Result<Vec<u8>> {
    let size_hint = K::size_hint(&key).unwrap_or(1024);
    let mut buf = Vec::with_capacity(prefix.len() + size_hint);
    buf.write(prefix)?;
    K::encode(&mut buf, key)?;
    Ok(buf)
}