use crate::buffer::Buffer;
use crate::dynamic::DynamicValue;
use crate::errors::{DecodeError, EncodeError};
use crate::field::Field;
use crate::kind::Kind;
use crate::value::Value;

#[non_exhaustive]
pub struct StructType<'a> {
    pub name: &'a str,
    pub fields: &'a [Field<'a>],
    pub sealed: bool,
}

pub trait StructCodec {
    const NAME: &'static str;
    // const NUM_FIELDS: usize;
    const FIELDS: &'static [Field<'static>];
    const SEALED: bool;
    type Borrowed<'a>: 'a;
    fn get_field(value: Self::Borrowed, index: usize) -> Result<DynamicValue, ()>;
    // fn set_field<'a>(&'a mut self, index: usize, value: DynamicValue<'a>) -> Result<(), EncodeError>;
    fn from_fields<'a>(fields: &'a [DynamicValue]) -> Result<Self::Borrowed<'a>, DecodeError>;
}

impl<V: StructCodec> Value for V {
    type Borrowed<'a> = <V as StructCodec>::Borrowed<'a>;
    const KIND: Kind = Default::default();

    fn to_dynamic(value: Self::Borrowed) -> DynamicValue {
        DynamicValue::Struct(value)
    }

    fn from_dynamic<'a>(value: &'a DynamicValue<'a>) -> Result<Self::Borrowed<'a>, DecodeError> {
        match value {
            DynamicValue::Struct(value) => Ok(*value),
            _ => Err(DecodeError::InvalidKind { expected: Kind::Struct, got: value.kind() }),
        }
    }
}