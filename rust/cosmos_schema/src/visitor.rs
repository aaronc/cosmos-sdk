use crate::kind::{ListElementKind, Type};
use crate::list::{ListAppender, ListReader};
use crate::r#struct::StructCodec;
use crate::value::Value;

pub trait Encoder {
    fn encode_i32(&mut self, value: i32) -> Result<(), EncodeError>;
    fn encode_str(&mut self, value: &str) -> Result<(), EncodeError>;
    fn encode_struct<V: StructCodec>(&mut self, value: &V) -> Result<(), EncodeError>;
    fn encode_list<'a, EK: ListElementKind>(&mut self, value: &dyn ListReader<'a, EK>) -> Result<(), EncodeError>;
    // fn encode_enum(&mut self, value: i32) -> Result<(), EncodeError>;
}

pub enum EncodeError {
    InvalidFieldIndex { index: usize },
}

pub trait Decoder<'a>
where
    Self: 'a,
{
    fn decode_i32(&mut self) -> Result<i32, DecodeError>;
    fn decode_u32(&mut self) -> Result<u32, DecodeError>;
    fn decode_str(&mut self) -> Result<&'a str, DecodeError>;
    fn decode_struct<'b, V: StructCodec + 'a>(&'b mut self, v: &'a mut V) -> Result<(), DecodeError>;
    fn decode_list<'b, EK: ListElementKind>(&'b mut self, v: &'b mut dyn ListAppender<'a, 'b, EK>) -> Result<(), DecodeError>;
    // fn decode_enum(&mut self) -> Result<i32, DecodeError>;
}

pub enum DecodeError {
    InvalidUtf8,
    InvalidFieldIndex { index: usize },
}

pub fn encode_value<'a, 'b, E: Encoder, K: Type, V: Value<'a, K>>(encoder: &'b mut E, value: &'b V) -> Result<(), EncodeError> {
    K::encode(encoder, value.to_encode_value())
}

pub fn decode_value<'a:'b, 'b, D: Decoder<'a>, K: Type, V: Value<'a, K>>(decoder: &'b mut D, value: &'b mut V) -> Result<(), DecodeError> {
    value.decode(decoder)
}
