use crate::field::Field;
use crate::kind::Kind;
use crate::value::Value;
use crate::visitor::{Decoder, DecodeError, Encoder, EncodeError};

#[non_exhaustive]
pub struct StructType<'a> {
    pub name: &'a str,
    pub fields: &'a [Field<'a>],
    pub sealed: bool,
}

pub unsafe trait StructCodec {
    const NAME: &'static str;
    // const NUM_FIELDS: usize;
    const FIELDS: &'static [Field<'static>];
    const SEALED: bool;
    type MaybeBorrowed<'a>: 'a;
    fn encode_field<'a, V: Encoder<'a>>(value: Self::MaybeBorrowed<'a>, index: usize, visitor: &'a mut V) -> Result<(), EncodeError>;
    fn decode<'a, V: Decoder<'a>>(visitor: &'a mut V) -> Result<Self::MaybeBorrowed<'a>, DecodeError>;
    // fn get_field(value: Self::Borrowed, index: usize) -> Result<DynamicValue, ()>;
    // // fn set_field<'a>(&'a mut self, index: usize, value: DynamicValue<'a>) -> Result<(), EncodeError>;
    // fn from_fields<'a>(fields: &'a [DynamicValue]) -> Result<Self::Borrowed<'a>, DecodeError>;
}

impl<V> Value for V
where
    V: StructCodec,
{
    type MaybeBorrowed<'a> = <V as StructCodec>::MaybeBorrowed<'a>;
    const KIND: Kind = Kind::Struct;

    fn encode<'a, E: Encoder<'a>>(value: Self::MaybeBorrowed<'a>, encoder: &'a mut E) -> Result<(), EncodeError> {
        encoder.visit_struct(value)
    }

    fn decode<'a, D: Decoder<'a>>(decoder: &'a mut D) -> Result<Self::MaybeBorrowed<'a>, DecodeError> {
        decoder.read_struct::<V>()
    }

    // fn to_dynamic(value: Self::Borrowed) -> DynamicValue {
    //     DynamicValue::Struct(value)
    // }
    //
    // fn from_dynamic<'a>(value: &'a DynamicValue<'a>) -> Result<Self::Borrowed<'a>, DecodeError> {
    //     match value {
    //         DynamicValue::Struct(value) => Ok(*value),
    //         _ => Err(DecodeError::InvalidKind { expected: Kind::Struct, got: value.kind() }),
    //     }
    // }
}