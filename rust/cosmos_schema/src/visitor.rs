use crate::kind::{ListElementKind, Type};
use crate::list::{ListAppender, ListReader};
use crate::r#struct::StructCodec;
use crate::value::Value;

pub trait Encoder {
    fn encode_i32(&mut self, value: i32) -> Result<(), EncodeError>;
    fn encode_str(&mut self, value: &str) -> Result<(), EncodeError>;
    fn encode_struct<V: StructCodec>(&mut self, value: &V) -> Result<(), EncodeError>;
    fn encode_list<'a:'b, 'b, EK: ListElementKind>(&mut self, value: &'b dyn ListReader<'a, 'b, EK>) -> Result<(), EncodeError>;
    // fn encode_enum(&mut self, value: i32) -> Result<(), EncodeError>;
}

pub enum EncodeError {
    InvalidFieldIndex { index: usize },
}

pub trait Decoder<'a>
{
    fn decode_i32(&mut self) -> Result<i32, DecodeError>;
    fn decode_u32(&mut self) -> Result<u32, DecodeError>;
    fn decode_str(&mut self) -> Result<&'a str, DecodeError>;
    fn decode_struct<'b, V: StructCodec + 'a>(&'b mut self, v: &'b mut V) -> Result<(), DecodeError>;
    fn decode_list<'b, EK: ListElementKind>(&'b mut self, v: &'b mut dyn ListAppender<'a, 'b, EK>) -> Result<(), DecodeError>;
    // fn decode_enum(&mut self) -> Result<i32, DecodeError>;
}

pub enum DecodeError {
    InvalidUtf8,
    InvalidFieldIndex { index: usize },
}

// pub fn encode_value<'a, E: Encoder, K: Type, V: Value<'a, K>>(encoder: &mut E, value: &'a V) -> Result<(), EncodeError>
//     where <K as Type>::SetType<'a>: 'a {
//     // K::encode(encoder, V::to_encode_value(value))
//     todo!()
// }
//
// pub fn decode_value<'a, 'b, D: Decoder<'a>, K: Type, V: Value<'a, K>>(decoder: &'b mut D, value: &'a mut V) -> Result<(), DecodeError>
// where <K as Type>::SetType<'a>: 'a {
//     // V::decode(value, |set_value| Ok(K::decode(decoder, set_value)?))
//     todo!()
// }
