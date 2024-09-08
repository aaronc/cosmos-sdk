use crate::field::Field;
use crate::kind::StructKind;
use crate::value::Value;
use crate::visitor::{DecodeError, Decoder, EncodeError, Encoder};

#[non_exhaustive]
pub struct StructType<'a> {
    pub name: &'a str,
    pub fields: &'a [Field<'a>],
    pub sealed: bool,
}

pub unsafe trait StructCodec<'a> {
    const NAME: &'static str;
    // const NUM_FIELDS: usize;
    const FIELDS: &'static [Field<'static>];
    const SEALED: bool;
    const FIELD_HAS_DEFAULT_MASK: &'static [u8];
    fn field_encoder<V: Encoder<'a>>(index: usize) -> Result<StructFieldEncoder<'a, Self, V>, EncodeError>;
    fn field_decoder<V: Decoder<'a>>(index: usize) -> Result<StructFieldDecoder<'a, Self, V>, DecodeError>;
    unsafe fn unsafe_init_default() -> Self;
}

pub type StructFieldEncoder<'a, S: StructCodec<'a>, E: Encoder<'a>> = fn(&'a S, &'a mut E) -> Result<(), EncodeError>;

pub type StructFieldDecoder<'a, S: StructCodec<'a>, D: Decoder<'a>> = fn(&'a mut S, &'a mut D) -> Result<(), DecodeError>;

impl<'a, S: StructCodec<'a> + Sized + 'a> Value<'a, StructKind<S>> for S {
    fn to_encode_value(&'a self) -> &'a S { self }
    fn from_decode_value(value: S) -> Self { value }
}

// impl<V> Value for V
// where
//     V: StructCodec,
// {
//     type MaybeBorrowed<'a> = <V as StructCodec>::MaybeBorrowed<'a>;
//     const KIND: Kind = Kind::Struct;
//
//     fn encode<'a, E: Encoder<'a>>(value: Self::MaybeBorrowed<'a>, encoder: &'a mut E) -> Result<(), EncodeError> {
//         encoder.visit_struct(value)
//     }
//
//     fn decode<'a, D: Decoder<'a>>(decoder: &'a mut D) -> Result<Self::MaybeBorrowed<'a>, DecodeError> {
//         decoder.read_struct::<V>()
//     }
//
//     // fn to_dynamic(value: Self::Borrowed) -> DynamicValue {
//     //     DynamicValue::Struct(value)
//     // }
//     //
//     // fn from_dynamic<'a>(value: &'a DynamicValue<'a>) -> Result<Self::Borrowed<'a>, DecodeError> {
//     //     match value {
//     //         DynamicValue::Struct(value) => Ok(*value),
//     //         _ => Err(DecodeError::InvalidKind { expected: Kind::Struct, got: value.kind() }),
//     //     }
//     // }
// }