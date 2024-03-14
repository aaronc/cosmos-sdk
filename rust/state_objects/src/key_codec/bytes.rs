use cosmossdk_core::mem::Ref;
use crate::key_codec::{KeyCodec, KeyPartCodec, Writer};

impl KeyCodec for [u8] {
    type Borrowed<'a> = &'a [u8];
    type AsRef<'a> = Ref<'a, &'a [u8]>;

    fn encode<B: Writer>(buf: &mut B, key: Self::Borrowed<'_>) -> cosmossdk_core::Result<()> {
        buf.write(key)
    }

    fn decode<'a>(buf: &'a [u8]) -> cosmossdk_core::Result<(Self::Borrowed<'a>, usize)> {
        Ok((buf, buf.len()))
    }

    fn as_ref<'a>(borrowed: Self::Borrowed<'a>, r: Ref<'a, &'a [u8]>) -> Self::AsRef<'a> {
        r.cast(borrowed)
    }
}

impl KeyPartCodec for [u8] {
    fn decode_non_terminal<'a>(buf: &'a [u8]) -> cosmossdk_core::Result<(Self::Borrowed<'a>, usize)> {
        todo!()
    }

    fn encode_non_terminal<B: Writer>(buf: &mut B, key: Self::Borrowed<'_>) -> cosmossdk_core::Result<()> {
        todo!()
    }
}