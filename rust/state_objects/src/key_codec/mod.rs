use cosmossdk_core::mem::Ref;
use cosmossdk_core::Result;
use crate::buffer::{Writer};

mod tuple;
mod bytes;
mod uint;
mod int;
mod bool;
mod str;

pub trait KeyCodec {
    type Borrowed<'a>: 'a;
    type AsRef<'a>;
    // type Keys<'a>;

    fn encode<B: Writer>(buf: &mut B, key: Self::Borrowed<'_>) -> Result<()>;

    fn decode<'a>(buf: &'a [u8]) -> Result<(Self::Borrowed<'a>, usize)>;

    fn size_hint(key: &Self::Borrowed<'_>) -> Option<usize> { None }

    fn as_ref<'a>(borrowed: Self::Borrowed<'a>, r: Ref<'a, &'a [u8]>) -> Self::AsRef<'a>;
}

pub trait KeyPartCodec: KeyCodec {
    fn encode_non_terminal<B: Writer>(buf: &mut B, key: Self::Borrowed<'_>) -> Result<()> {
        Self::encode(buf, key)
    }

    fn decode_non_terminal<'a>(buf: &'a [u8]) -> Result<(Self::Borrowed<'a>, usize)> {
        Self::decode(buf)
    }
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