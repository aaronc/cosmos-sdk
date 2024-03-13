use cosmossdk_core::mem::Ref;
use crate::key_codec::{KeyCodec, Reader, Writer};

impl KeyCodec for u32 {
    type Borrowed<'a> = u32;
    type AsRef<'a> = u32;

    fn encode<B: Writer>(buf: &mut B, key: Self::Borrowed<'_>) -> cosmossdk_core::Result<()> {
        buf.write(&key.to_be_bytes())
    }

    fn decode<'a, B: Reader>(buf: &'a mut B) -> cosmossdk_core::Result<Self::Borrowed<'a>> {
        let bz = buf.read(4)?;
        Ok(u32::from_be_bytes([bz[0], bz[1], bz[2], bz[3]]))
    }

    fn as_ref<'a>(borrowed: Self::Borrowed<'a>, _r: Ref<'a, &'a [u8]>) -> Self::AsRef<'a> {
        borrowed
    }

    fn size_hint(key: &Self::Borrowed<'_>) -> Option<usize> {
        Some(4)
    }
}