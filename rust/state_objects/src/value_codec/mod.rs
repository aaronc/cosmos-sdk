mod bytes;

use cosmossdk_core::mem::Ref;
use crate::buffer::{Writer};

pub trait ValueCodec {
    type Borrowed<'a>;
    type AsRef<'a>;
    // type Keys<'a>;

    fn encode<B: Writer>(buf: &mut B, value: Self::Borrowed<'_>) -> cosmossdk_core::Result<()>;

    fn decode<'a>(buf: &'a [u8]) -> cosmossdk_core::Result<(Self::Borrowed<'a>, usize)>;

    fn as_ref<'a>(borrowed: Self::Borrowed<'a>, r: Ref<'a, &'a [u8]>) -> Self::AsRef<'a>;
    //     r.cast(borrowed)
    // }

    fn size_hint(key: &Self::Borrowed<'_>) -> Option<usize> { None }
}