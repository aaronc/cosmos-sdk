use cosmossdk_core::Result;
use std::borrow::Borrow;
use cosmossdk_core::mem::Ref;
use crate::key_codec::{KeyCodec, PrefixKey, Writer};

pub trait KeyPartCodec: KeyCodec {
    fn encode_non_terminal<B: Writer>(buf: &mut B, key: Self::Borrowed<'_>) -> Result<()> {
        Self::encode(buf, key)
    }

    fn decode_non_terminal<'a>(buf: &'a [u8]) -> Result<(Self::Borrowed<'a>, usize)> {
        Self::decode(buf)
    }
}

impl<A: KeyPartCodec, B: KeyPartCodec> KeyCodec for (A, B) {
    type Borrowed<'a> = (A::Borrowed<'a>, B::Borrowed<'a>);
    type AsRef<'a> = Ref<'a, Self::Borrowed<'a>>;

    fn encode<Buf: Writer>(buf: &mut Buf, key: Self::Borrowed<'_>) -> cosmossdk_core::Result<()> {
        A::encode_non_terminal(buf, key.0)?;
        B::encode(buf, key.1)
    }

    fn decode<'a>(buf: &'a [u8]) -> Result<(Self::Borrowed<'a>, usize)> {
        let (a, n) = A::decode_non_terminal(buf)?;
        let (b, n) = B::decode(&buf[n..])?;
        Ok(((a, b), n))
    }

    fn as_ref<'a>(borrowed: Self::Borrowed<'a>, r: Ref<'a, &'a [u8]>) -> Self::AsRef<'a> {
        r.cast(borrowed)
    }
}

// impl<A: KeyPartCodec, B: KeyPartCodec> PrefixKey<(A, B)> for (A, ) {}

// impl<A: KeyPartCodec, B: KeyPartCodec, C: KeyPartCodec> KeyCodec for (A, B, C) {
//     type Borrowed<'a> = (A::Borrowed<'a>, B::Borrowed<'a>, C::Borrowed<'a>);
//     type AsRef<'a> = (A::AsRef<'a>, B::AsRef<'a>, C::AsRef<'a>);
//
//     fn encode<Buf: Writer>(buf: &mut Buf, key: Self::Borrowed<'_>) -> cosmossdk_core::Result<()> {
//         A::encode_non_terminal(buf, key.0)?;
//         B::encode_non_terminal(buf, key.1)?;
//         C::encode(buf, key.2)
//     }
//
//     fn decode<Buf: Reader>(buf: &Buf) -> cosmossdk_core::Result<Self::Borrowed<'_>> {
//         let a = A::decode_non_terminal(buf)?;
//         let b = B::decode(buf)?;
//         let c = C::decode(buf)?;
//         Ok((a, b, c))
//     }
// }
//
// impl<A: KeyPartCodec, B: KeyPartCodec, C: KeyPartCodec> PrefixKey<(A, B, C)> for (A, ) {}
//
// impl<A: KeyPartCodec, B: KeyPartCodec, C: KeyPartCodec> PrefixKey<(A, B, C)> for (A, B) {}
