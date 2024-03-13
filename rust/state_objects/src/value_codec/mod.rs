use cosmossdk_core::mem::Ref;
use crate::buffer::{Reader, Writer};

pub trait ValueCodec {
    type Borrowed<'a>;
    type AsRef<'a>;
    // type Keys<'a>;

    fn encode<B: Writer>(buf: &mut B, key: Self::Borrowed<'_>) -> cosmossdk_core::Result<()>;

    fn decode<B: Reader>(buf: &B) -> cosmossdk_core::Result<Self::Borrowed<'_>>;

    fn as_ref<'a>(borrowed: Self::Borrowed<'a>, r: Ref<'a, &'a [u8]>) -> Self::AsRef<'a>;
    //     r.cast(borrowed)
    // }
}