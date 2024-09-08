use crate::kind::Kind;
use crate::visitor::{Decoder, DecodeError, Encoder, EncodeError};

pub trait Value {
    type MaybeBorrowed<'a>;
    const KIND: Kind;
    const NULLABLE: bool = false;
    fn encode<'a, V: Encoder<'a>>(value: Self::MaybeBorrowed<'a>, visitor: &'a mut V) -> Result<(), EncodeError>;
    fn decode<'a, V: Decoder<'a>>(visitor: &'a mut V) -> Result<Self::MaybeBorrowed<'a>, DecodeError>;
    // fn to_dynamic(value: Self::Borrowed) -> DynamicValue;
    // fn from_dynamic<'a>(value: &'a DynamicValue<'a>) -> Result<Self::Borrowed<'a>, DecodeError>;
}

// impl Value for u8 {
//     type Borrowed<'a> = u8;
//     const KIND: Kind = Kind::Uint8;
// }
//
// impl Value for u16 {
//     type Borrowed<'a> = u16;
//     const KIND: Kind = Kind::Uint16;
// }
//
// impl Value for u32 {
//     type Borrowed<'a> = u32;
//     const KIND: Kind = Kind::Uint32;
// }
//
// impl Value for u64 {
//     type Borrowed<'a> = u64;
//     const KIND: Kind = Kind::Uint64;
// }
//
// impl Value for i8 {
//     type Borrowed<'a> = i8;
//     const KIND: Kind = Kind::Int8;
// }
//
// impl Value for i16 {
//     type Borrowed<'a> = i16;
//     const KIND: Kind = Kind::Int16;
// }
//
impl Value for i32 {
    type MaybeBorrowed<'a> = i32;
    const KIND: Kind = Kind::Int32;

    fn encode<'a, V: Encoder<'a>>(value: Self::MaybeBorrowed<'a>, visitor: &'a mut V<'a>) -> Result<(), EncodeError> {
        visitor.visit_i32(value)
    }

    fn decode<'a, V: Decoder<'a>>(visitor: &'a mut V) -> Result<Self::MaybeBorrowed<'a>, DecodeError> {
        visitor.read_i32()
    }

    // fn to_dynamic(value: i32) -> DynamicValue {
    //     DynamicValue::I32(value)
    // }
    //
    // fn from_dynamic(value: &DynamicValue) -> Result<Self, DecodeError> {
    //     match value {
    //         DynamicValue::I32(value) => Ok(*value),
    //         _ => Err(DecodeError::InvalidKind { expected: Kind::Int32, got: value.kind() }),
    //     }
    // }
}
//
// impl Value for i64 {
//     type Borrowed<'a> = i64;
//     const KIND: Kind = Kind::Int64;
// }
//
// impl Value for bool {
//     type Borrowed<'a> = bool;
//     const KIND: Kind = Kind::Bool;
// }
//
impl Value for str {
    type MaybeBorrowed<'a> = &'a str;
    const KIND: Kind = Kind::String;

    fn encode<'a, V: Encoder<'a>>(value: Self::MaybeBorrowed<'a>, visitor: &'a mut V) -> Result<(), EncodeError> {
        visitor.visit_str(value)
    }

    fn decode<'a, V: Decoder<'a>>(visitor: &'a mut V) -> Result<Self::MaybeBorrowed<'a>, DecodeError> {
        visitor.read_str()
    }

    // fn to_dynamic(value: &str) -> DynamicValue {
    //     DynamicValue::String(value)
    // }
    //
    // fn from_dynamic<'a>(value: &'a DynamicValue<'a>) -> Result<&'a str, DecodeError> {
    //     match value {
    //         DynamicValue::String(value) => Ok(value),
    //         _ => Err(DecodeError::InvalidKind { expected: Kind::String, got: value.kind() }),
    //     }
    // }
}
//
// impl <V: Value> Value for Option<V> {
//     type Borrowed<'a> = Option<V::Borrowed<'a>>;
//     const KIND: Kind = V::KIND;
//     const NULLABLE: bool = true;
// }