use crate::kind::{ListElementKind, TypeLevelKind};
use crate::list::{ListAppender, ListCodec};
use crate::r#struct::StructCodec;
use crate::value::Value;

pub trait Encoder {
    fn encode_i32(&mut self, value: i32) -> Result<(), EncodeError>;
    fn encode_str(&mut self, value: &str) -> Result<(), EncodeError>;
    fn encode_struct<'a, V: StructCodec<'a>>(&mut self, value: &'a V) -> Result<(), EncodeError>;
    fn encode_enum(&mut self, value: i32) -> Result<(), EncodeError>;
}

pub enum EncodeError {
    InvalidFieldIndex { index: usize },
}

pub trait Decoder<'a> {
    fn decode_i32(&mut self) -> Result<i32, DecodeError>;
    fn decode_u32(&mut self) -> Result<u32, DecodeError>;
    fn decode_str(&mut self) -> Result<&'a str, DecodeError>;
    fn decode_struct<'b, V: StructCodec<'a>>(&mut self, v: &'b mut V) -> Result<(), DecodeError>;
    fn decode_list<V: ListCodec<'a, EK>, EK: ListElementKind<'a>>(&mut self, v: &'a mut V) -> Result<(), DecodeError>;
    fn decode_list2<EK: ListElementKind<'a>>(&mut self, v: &'a mut dyn ListAppender<'a, EK>) -> Result<(), DecodeError>;
    fn decode_enum(&mut self) -> Result<i32, DecodeError>;
}

pub enum DecodeError {
    InvalidUtf8,
    InvalidFieldIndex { index: usize },
}

pub fn encode_value<'a, E: Encoder, K: TypeLevelKind<'a>, V: Value<'a, K>>(encoder: &mut E, value: &'a V) -> Result<(), EncodeError> {
    K::encode(encoder, V::to_encode_value(value))
}

pub fn decode_value<'a, 'b, D: Decoder<'a>, K: TypeLevelKind<'a>, V: Value<'a, K>>(decoder: &'b mut D) -> Result<V, DecodeError> {
    Ok(V::from_decode_value(K::decode(decoder)?))
}
