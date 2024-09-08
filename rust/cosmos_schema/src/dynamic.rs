use crate::kind::Kind;
use crate::r#struct::StructCodec;

pub enum DynamicValue<'a> {
    Null,
    I8(i8),
    U8(u8),
    I6(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    Bool(bool),
    String(&'a str),
    Bytes(&'a [u8]),
    Address(&'a [u8]),
    Enum(i32),
    Struct(&'a dyn StructCodec::Borrowed<'a>),
}

impl<'a> DynamicValue<'a> {
    pub fn kind(&self) -> Kind {
        todo!()
    }
}