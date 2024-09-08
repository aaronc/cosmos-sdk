use crate::kind::{I32Kind, Kind, NullablePseudoKind, StringKind, TypeLevelKind};
use crate::visitor::{Decoder, DecodeError, Encoder, EncodeError};

pub trait Value<'a, K: TypeLevelKind<'a>> {
    fn to_encode_value(&'a self) -> K::EncodeType;
    fn from_decode_value(value: K::DecodeType) -> Self;
}

impl Value<'_, I32Kind> for i32 {
    fn to_encode_value(&self) -> i32 { *self }
    fn from_decode_value(value: i32) -> Self { value }
}

impl <'a> Value<'a, StringKind> for &'a str {
    fn to_encode_value(&self) -> &str { self }
    fn from_decode_value(value: &'a str) -> &'a str { value }
}

impl <'a, K: TypeLevelKind<'a>, V: Value<'a, K> + Sized> Value<'a, NullablePseudoKind<'a, K>> for Option<V> {
    fn to_encode_value(&'a self) -> Option<K::EncodeType> {
        self.as_ref().map(|v| v.to_encode_value())
    }

    fn from_decode_value(value: Option<K::DecodeType>) -> Self {
        value.map(|v| V::from_decode_value(v))
    }
}