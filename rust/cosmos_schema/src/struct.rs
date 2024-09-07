use crate::buffer::Buffer;
use crate::errors::{DecodeError, EncodeError};
use crate::field::Field;

pub struct StructType<'a> {
    pub name: &'a str,
    pub fields: &'a [Field<'a>],
    pub sealed: bool,
}

pub trait StructCodec {
    const SCHEMA: StructType<'static>;
    fn encode<B: Buffer>(&self, buf: &mut B) -> Result<(), EncodeError>;
    fn decode(buf: &[u8]) -> Result<(Self, usize), DecodeError>;
    fn size_hint(&self) -> Option<usize>;
}