use crate::kind::{I32Type, Kind, NullablePseudoKind, StringType, Type};
use crate::visitor::{Decoder, DecodeError, Encoder, EncodeError};

pub trait Value<'a, K: Type> {
    fn to_encode_value(&'a self) -> K::EncodeType<'a>;
    fn from_decode_value(value: K::DecodeType<'a>) -> Self;
}

impl Value<'_, I32Type> for i32 {
    fn to_encode_value(&self) -> i32 { *self }
    fn from_decode_value(value: i32) -> Self { value }
}

impl<'a> Value<'a, StringType> for &'a str {
    fn to_encode_value(&self) -> &str { self }
    fn from_decode_value(value: &'a str) -> Self { value }
}

#[cfg(not(feature = "no_std"))]
impl<'a> Value<'a, StringType> for String {
    fn to_encode_value(&self) -> &str { self.as_str() }
    fn from_decode_value(value: &'a str) -> Self { value.to_string() }
}

impl<'a, K: Type, V: Value<'a, K> + Sized> Value<'a, NullablePseudoKind<K>> for Option<V> {
    fn to_encode_value(&'a self) -> Option<K::EncodeType<'a>> {
        self.as_ref().map(|v| v.to_encode_value())
    }

    fn from_decode_value(value: Option<K::DecodeType<'a>>) -> Self {
        value.map(|v| V::from_decode_value(v))
    }
}