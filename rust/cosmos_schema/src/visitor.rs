use crate::r#struct::StructCodec;

pub trait Encoder<'a> {
    fn visit_i32(&'a mut self, value: i32) -> Result<(), EncodeError>;
    fn visit_str(&'a mut self, value: &'a str) -> Result<(), EncodeError>;
    fn visit_struct<V: StructCodec>(&'a mut self, value: &'a StructCodec::MaybeBorrowed<'a>) -> Result<(), EncodeError>;
    fn visit_enum(&'a mut self, value: i32) -> Result<(), EncodeError>;
}

pub enum EncodeError {
    InvalidFieldIndex { index: usize },
}

pub trait Decoder<'a> {
    fn read_i32(&'a mut self) -> Result<i32, DecodeError>;
    fn read_u32(&'a mut self) -> Result<u32, DecodeError>;
    fn read_str(&'a mut self) -> Result<&'a str, DecodeError>;
    fn read_struct<V: StructCodec>(&'a mut self) -> Result<V::MaybeBorrowed<'a>, DecodeError>;
    fn read_enum(&'a mut self) -> Result<i32, DecodeError>;
}

pub enum DecodeError {
    InvalidUtf8,
}