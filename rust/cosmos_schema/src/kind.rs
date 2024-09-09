use num_enum::{FromPrimitive, IntoPrimitive};
use crate::enum_type::{EnumCodec, EnumKind, EnumType, EnumValueDefinition};
use crate::list::{ListAppender, ListCodec};
use crate::r#struct::StructCodec;
use crate::value::Value;
use crate::visitor::{DecodeError, Decoder, EncodeError, Encoder};

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
    IntN,
    UIntN,
    Decimal,
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

// TODO: rename this to something else, maybe simply Type because it's everything in field except Name basically
pub trait TypeLevelKind<'a>: Private {
    const KIND: Kind;
    const NULLABLE: bool = false;
    const SIZE_LIMIT: Option<u32> = None;
    type EncodeType;
    type DecodeType;
    fn encode<E: Encoder>(encoder: &mut E, value: Self::EncodeType) -> Result<(), EncodeError>;
    fn decode<D: Decoder<'a>>(decoder: &mut D) -> Result<Self::DecodeType, DecodeError>;
}

impl Private for I32Kind {}
impl<'a> TypeLevelKind<'a> for I32Kind {
    const KIND: Kind = Kind::String;
    type EncodeType = i32;
    type DecodeType = i32;

    fn encode<E: Encoder>(encoder: &mut E, value: Self::EncodeType) -> Result<(), EncodeError> {
        encoder.encode_i32(value)
    }

    fn decode<D: Decoder<'a>>(decoder: &mut D) -> Result<Self::DecodeType, DecodeError> {
        decoder.decode_i32()
    }
}

impl Private for StringKind {}
impl<'a> TypeLevelKind<'a> for StringKind {
    const KIND: Kind = Kind::String;
    type EncodeType = &'a str;
    type DecodeType = &'a str;

    fn encode<E: Encoder>(encoder: &mut E, value: Self::EncodeType) -> Result<(), EncodeError> {
        encoder.encode_str(value)
    }

    fn decode<D: Decoder<'a>>(decoder: &mut D) -> Result<Self::DecodeType, DecodeError> {
        decoder.decode_str()
    }
}

impl<S> Private for StructKind<S> {}
impl<'a, S: StructCodec<'a> + Sized + 'a> TypeLevelKind<'a> for StructKind<S> {
    const KIND: Kind = Kind::Struct;
    type EncodeType = &'a S;
    type DecodeType = S;

    fn encode<E: Encoder>(encoder: &mut E, value: Self::EncodeType) -> Result<(), EncodeError> {
        encoder.encode_struct(value)
    }

    fn decode<'b, D: Decoder<'a>>(decoder: &'b mut D) -> Result<Self::DecodeType, DecodeError> {
        unsafe {
            let mut s = S::unsafe_init_default();
            decoder.decode_struct::<S>(&mut s)?;
            // TODO check for missing default values with FIELD_HAS_DEFAULT_MASK
            Ok(s)
        }
    }
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

    fn encode<E: Encoder>(encoder: &mut E, value: Self::EncodeType) -> Result<(), EncodeError> {
        todo!()
    }

    fn decode<D: Decoder<'a>>(decoder: &mut D) -> Result<Self::DecodeType, DecodeError> {
        todo!()
    }
}

// impl Private for ListKind {}
// impl TypeLevelKind for ListKind {
//     const KIND: Kind = Kind::List;
// }

trait Private {}

// TODO remove L, we should just need to specify EK here because the list type is not a fundamentally
// a different type in the type system, but List<EK> is
pub struct ListKind<EK> {
    _phantom2: std::marker::PhantomData<EK>,
}

impl<EK> Private for ListKind<EK> {}
impl<'a, EK: ListElementKind<'a>> TypeLevelKind<'a> for ListKind<EK> {
    const KIND: Kind = Kind::List;
    type EncodeType = todo!();
    type DecodeType = &'a mut dyn ListAppender<'a, EK>;

    fn encode<E: Encoder>(encoder: &mut E, value: Self::EncodeType) -> Result<(), EncodeError> {
        todo!()
    }

    fn decode<D: Decoder<'a>>(decoder: &mut D) -> Result<Self::DecodeType, DecodeError> {
        todo!()
    }
}

pub trait ListElementKind<'a>: TypeLevelKind<'a> {}
impl<'a> ListElementKind<'a> for I32Kind {}
impl<'a> ListElementKind<'a> for StringKind {}
impl<'a, S: StructCodec<'a> + 'a> ListElementKind<'a> for StructKind<S> {}

pub struct IntN<const N: u32>;
impl<const N: u32> Private for IntN<N> {}
impl<'a, const N: u32> TypeLevelKind<'a> for IntN<N> {
    const KIND: Kind = Kind::IntN;
    const SIZE_LIMIT: Option<u32> = Some(N);
    type EncodeType = [u8; N as usize];
    type DecodeType = [u8; N as usize];

    fn encode<E: Encoder>(encoder: &mut E, value: Self::EncodeType) -> Result<(), EncodeError> {
        todo!()
    }

    fn decode<D: Decoder<'a>>(decoder: &mut D) -> Result<Self::DecodeType, DecodeError> {
        todo!()
    }
}

pub struct UIntN<const N: u32>;
