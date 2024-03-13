use cosmossdk_core::mem::Ref;
use crate::key_codec::{KeyCodec, Reader, Writer};

impl KeyCodec for [u8] {
    type Borrowed<'a> = &'a [u8];
    type AsRef<'a> = Ref<'a, &'a [u8]>;

    fn encode<B: Writer>(buf: &mut B, key: Self::Borrowed<'_>) -> cosmossdk_core::Result<()> {
        buf.write(key)
    }

    fn decode<'a, B: Reader>(buf: &'a mut B) -> cosmossdk_core::Result<Self::Borrowed<'a>> {
        buf.read_all()
    }

    fn as_ref<'a>(borrowed: Self::Borrowed<'a>, r: Ref<'a, &'a [u8]>) -> Self::AsRef<'a> {
        r.cast(borrowed)
    }
}