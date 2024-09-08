use num_enum::{FromPrimitive, IntoPrimitive};
use crate::enum_type::{EnumCodec, EnumKind, EnumType, EnumValueDefinition};
use crate::r#struct::StructCodec;

#[non_exhaustive]
#[repr(u32)]
#[derive(FromPrimitive, IntoPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    String = 1,
    Bytes = 2,
    Int8 = 3,
    Uint8 = 4,
    Int16 = 5,
    Uint16 = 6,
    Int32 = 7,
    Uint32 = 8,
    Int64 = 9,
    Uint64 = 10,
    IntegerString,
    DecimalString,
    Bool,
    Time,
    Duration,
    Float32,
    Float64,
    Address,
    Enum,
    JSON,
    Struct,
    List,
    #[num_enum(catch_all)]
    Unknown(u32),
}

// unsafe impl EnumCodec for Kind {
//     const NAME: &'static str = "Kind";
//     const VALUES: &'static [EnumValueDefinition<'static>] = &[
//         EnumValueDefinition::new("String", Kind::String.into()),
//         EnumValueDefinition::new("Bytes", Kind::Bytes.into()),
//         EnumValueDefinition::new("Int8", Kind::Int8.into()),
//         EnumValueDefinition::new("Uint8", Kind::Uint8.into()),
//         EnumValueDefinition::new("Int16", Kind::Int16.into()),
//         EnumValueDefinition::new("Uint16", Kind::Uint16.into()),
//         // TODO
//     ];
// }

pub struct I32Kind;
pub struct StringKind;
pub struct StructKind<'a, S> {
    _phantom: std::marker::PhantomData<S>,
    _phantom_lifetime: std::marker::PhantomData<&'a ()>,
}
pub struct ListKind;

pub trait TypeLevelKind<'a>: Private {
    const KIND: Kind;
    type EncodeType;
    // where
    //     Self: 'a + Sized;
    type DecodeType;
    // where
    //     Self: 'a + Sized;
}

impl Private for I32Kind {}
impl TypeLevelKind<'_> for I32Kind {
    const KIND: Kind = Kind::String;
    type EncodeType = i32;
    type DecodeType = i32;
}

impl Private for StringKind {}
impl<'a> TypeLevelKind<'a> for StringKind {
    const KIND: Kind = Kind::String;
    type EncodeType = &'a str;
    type DecodeType = &'a str;
}

impl<'a, S> Private for StructKind<'a, S> {}
impl<'a, S: StructCodec<'a> + Sized + 'a> TypeLevelKind<'a> for StructKind<'a, S> {
    const KIND: Kind = Kind::Struct;
    type EncodeType = &'a S;
    type DecodeType = S;
}

// impl Private for ListKind {}
// impl TypeLevelKind for ListKind {
//     const KIND: Kind = Kind::List;
// }

trait Private {}