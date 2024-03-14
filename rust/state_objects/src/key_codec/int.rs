use cosmossdk_core::{Code, err};
use cosmossdk_core::mem::Ref;
use crate::key_codec::{KeyCodec, Writer};

impl KeyCodec for u32 {
    type Borrowed<'a> = u32;
    type AsRef<'a> = u32;

    fn encode<B: Writer>(buf: &mut B, key: Self::Borrowed<'_>) -> cosmossdk_core::Result<()> {
        buf.write(&key.to_be_bytes())
    }

    fn decode<'a>(buf: &'a [u8]) -> cosmossdk_core::Result<(u32, usize)> {
        if buf.len() < 4 {
            return err!(Code::OutOfRange)
        }
        Ok((u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]), 4))
    }

    fn as_ref<'a>(borrowed: Self::Borrowed<'a>, _r: Ref<'a, &'a [u8]>) -> Self::AsRef<'a> {
        borrowed
    }

    fn size_hint(key: &Self::Borrowed<'_>) -> Option<usize> {
        Some(4)
    }
}