use crate::kind::{I32Type, Kind, NullablePseudoKind, StringType, Type};
use crate::visitor::{Decoder, DecodeError, Encoder, EncodeError};

pub trait Value<'a, K: Type<'a>> {
    fn to_encode_value(&'a self) -> K::EncodeType;
    fn from_decode_value(value: K::DecodeType) -> Self;
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

impl<'a, K: Type<'a>, V: Value<'a, K> + Sized> Value<'a, NullablePseudoKind<'a, K>> for Option<V> {
    fn to_encode_value(&'a self) -> Option<K::EncodeType> {
        self.as_ref().map(|v| v.to_encode_value())
    }

    fn from_decode_value(value: Option<K::DecodeType>) -> Self {
        value.map(|v| V::from_decode_value(v))
    }
}