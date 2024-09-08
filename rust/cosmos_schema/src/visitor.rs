use crate::kind::TypeLevelKind;
use crate::r#struct::StructCodec;
use crate::value::Value;

pub trait Encoder<'a> {
    fn encode_i32(&'a mut self, value: i32) -> Result<(), EncodeError>;
    fn encode_str(&'a mut self, value: &'a str) -> Result<(), EncodeError>;
    fn encode_struct<V: StructCodec<'a>>(&'a mut self, value: &'a V) -> Result<(), EncodeError>;
    fn visit_enum(&'a mut self, value: i32) -> Result<(), EncodeError>;
}

pub enum EncodeError {
    InvalidFieldIndex { index: usize },
}

pub trait Decoder<'a> {
    fn read_i32(&'a mut self) -> Result<i32, DecodeError>;
    fn read_u32(&'a mut self) -> Result<u32, DecodeError>;
    fn read_str(&'a mut self) -> Result<&'a str, DecodeError>;
    // fn read_struct<V: StructCodec<'a>>(&'a mut self) -> Result<V, DecodeError>;
    fn read_enum(&'a mut self) -> Result<i32, DecodeError>;
}

pub enum DecodeError {
    InvalidUtf8,
    InvalidFieldIndex { index: usize },
}

pub fn encode_value<'a, E: Encoder<'a>, K: TypeLevelKind<'a>, V: Value<'a, K>>(encoder: &'a mut E, value: &'a V) -> Result<(), EncodeError> {
    K::encode(encoder, V::to_encode_value(value))
}