use cosmossdk_core::Result;
use std::borrow::Borrow;
use crate::borrow::Ownable;
use crate::key_codec::{KeyCodec, PrefixKey, Writer};
use crate::dynamic::DynamicValue;

pub trait KeyPartCodec: KeyCodec + ToOwned {
    fn encode_non_terminal<B: Writer>(buf: &mut B, key: Self::Borrowed<'_>) -> Result<()> {
        Self::encode(buf, key)
    }

    fn decode_non_terminal<'a>(buf: &'a [u8]) -> Result<(Self::Borrowed<'a>, usize)> {
        Self::decode(buf)
    }

    fn size_hint_non_terminal(key: Self::Borrowed<'_>) -> usize {
        Self::size_hint(key)
    }
}

impl<A: KeyPartCodec, B: KeyPartCodec> KeyCodec for (A, B) {
    fn encode<Buf: Writer>(buf: &mut Buf, key: Self::Borrowed<'_>) -> cosmossdk_core::Result<()> {
        A::encode_non_terminal(buf, key.0)?;
        B::encode(buf, key.1)
    }

    fn decode<'a>(buf: &'a [u8]) -> Result<(Self::Borrowed<'a>, usize)> {
        let (a, n) = A::decode_non_terminal(buf)?;
        let (b, n) = B::decode(&buf[n..])?;
        Ok(((a, b), n))
    }

    fn size_hint(key: Self::Borrowed<'_>) -> usize {
        A::size_hint_non_terminal(key.0) + B::size_hint(key.1)
    }

    fn to_dynamic(key: Self::Borrowed<'_>) -> DynamicValue {
        todo!()
    }

    fn from_value(value: DynamicValue) -> Result<Self::Borrowed<'static>> {
        todo!()
    }
}

impl<A: KeyPartCodec, B: KeyPartCodec> PrefixKey<(A, B)> for (A, ) {}

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
