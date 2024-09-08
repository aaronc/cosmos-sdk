use num_enum::{FromPrimitive, IntoPrimitive};
use crate::enum_type::{EnumCodec, EnumKind, EnumType, EnumValueDefinition};
use crate::list::List;
use crate::r#struct::StructCodec;
use crate::value::Value;

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
pub struct StructKind<S> {
    _phantom: std::marker::PhantomData<S>,
    // _phantom_lifetime: std::marker::PhantomData<&'a ()>,
}

pub trait TypeLevelKind<'a>: Private {
    const KIND: Kind;
    const NULLABLE: bool = false;
    type EncodeType;
    type DecodeType;
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

impl<S> Private for StructKind<S> {}
impl<'a, S: StructCodec<'a> + Sized + 'a> TypeLevelKind<'a> for StructKind<S> {
    const KIND: Kind = Kind::Struct;
    type EncodeType = &'a S;
    type DecodeType = S;
}

pub struct NullablePseudoKind<'a, K> {
    _phantom: std::marker::PhantomData<K>,
    _phantom_lifetime: std::marker::PhantomData<&'a ()>,
}

impl<'a, K: TypeLevelKind<'a>> Private for NullablePseudoKind<'a, K> {}
impl<'a, K: TypeLevelKind<'a>> TypeLevelKind<'a> for NullablePseudoKind<'a, K> {
    const KIND: Kind = K::KIND;
    const NULLABLE: bool = true;
    type EncodeType = Option<K::EncodeType>;
    type DecodeType = Option<K::DecodeType>;
}

// impl Private for ListKind {}
// impl TypeLevelKind for ListKind {
//     const KIND: Kind = Kind::List;
// }

trait Private {}

pub struct ListKind<L, EK> {
    _phantom: std::marker::PhantomData<L>,
    _phantom2: std::marker::PhantomData<EK>,
}

impl<EK, L> Private for ListKind<L, EK> {}
impl<'a, EK: ListElementKind<'a>, L: List<'a, EK> + 'a> TypeLevelKind<'a> for ListKind<L, EK> {
    const KIND: Kind = Kind::List;
    type EncodeType = &'a L;
    type DecodeType = L;
}

pub trait ListElementKind<'a>: TypeLevelKind<'a> {}
impl<'a> ListElementKind<'a> for I32Kind {}
impl<'a> ListElementKind<'a> for StringKind {}
impl<'a, S: StructCodec<'a> + 'a> ListElementKind<'a> for StructKind<S> {}
