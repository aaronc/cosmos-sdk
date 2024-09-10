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

pub struct I32Type;
pub struct StringType;
pub struct StructKind<S> {
    _phantom: std::marker::PhantomData<S>,
    // _phantom_lifetime: std::marker::PhantomData<&'a ()>,
}

// TODO: rename this to something else, maybe simply Type because it's everything in field except Name basically
pub trait Type: Private {
    const KIND: Kind;
    const NULLABLE: bool = false;
    const SIZE_LIMIT: Option<u32> = None;
    const ELEMENT_KIND: Option<Kind> = None;
    type ReferencedType;

    type EncodeType<'a>;
    type DecodeType<'a>;

    fn encode<'a, E: Encoder>(encoder: &mut E, value: Self::EncodeType<'a>) -> Result<(), EncodeError>;
    fn decode<'a, D: Decoder<'a>>(decoder: &mut D) -> Result<Self::DecodeType<'a>, DecodeError>;
}

impl Private for I32Type {}
impl Type for I32Type {
    const KIND: Kind = Kind::String;
    type ReferencedType = ();
    type EncodeType = i32;
    type DecodeType = i32;

    fn encode<'a, E: Encoder>(encoder: &mut E, value: Self::EncodeType<'a>) -> Result<(), EncodeError> {
        encoder.encode_i32(value)
    }

    fn decode<'a, D: Decoder<'a>>(decoder: &mut D) -> Result<Self::DecodeType<'a>, DecodeError> {
        decoder.decode_i32()
    }
}

impl Private for StringType {}
impl Type for StringType {
    const KIND: Kind = Kind::String;
    type ReferencedType = ();
    type EncodeType<'a> = &'a str;
    type DecodeType<'a> = &'a str;

    fn encode<'a, E: Encoder>(encoder: &mut E, value: Self::EncodeType<'a>) -> Result<(), EncodeError> {
        encoder.encode_str(value)
    }

    fn decode<'a, D: Decoder<'a>>(decoder: &mut D) -> Result<Self::DecodeType<'a>, DecodeError> {
        decoder.decode_str()
    }
}

impl<S> Private for StructKind<S> {}
impl<'a, S: StructCodec<'a> + Sized + 'a> Type for StructKind<S> {
    const KIND: Kind = Kind::Struct;
    type EncodeType = &'a S;
    type DecodeType = S;
    type ReferencedType = S;

    fn encode<E: Encoder>(encoder: &mut E, value: Self::EncodeType<'a>) -> Result<(), EncodeError> {
        encoder.encode_struct(value)
    }

    fn decode<'b, D: Decoder<'a>>(decoder: &'b mut D) -> Result<Self::DecodeType<'a>, DecodeError> {
        unsafe {
            let mut s = S::unsafe_init_default();
            decoder.decode_struct::<S>(&mut s)?;
            // TODO check for missing default values with FIELD_HAS_DEFAULT_MASK
            Ok(s)
        }
    }
}

pub struct NullablePseudoKind<K> {
    _phantom: std::marker::PhantomData<K>,
}

impl<K: Type> Private for NullablePseudoKind<K> {}
impl<K: Type> Type for NullablePseudoKind<K> {
    const KIND: Kind = K::KIND;
    const NULLABLE: bool = true;
    type ReferencedType = ();
    type EncodeType<'a> = Option<K::EncodeType<'a>>;
    type DecodeType<'a> = Option<K::DecodeType<'a>>;

    fn encode<'a, E: Encoder>(encoder: &mut E, value: Self::EncodeType<'a>) -> Result<(), EncodeError> {
        todo!()
    }

    fn decode<'a, D: Decoder<'a>>(decoder: &mut D) -> Result<Self::DecodeType<'a>, DecodeError> {
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
impl<'a, EK: ListElementKind<'a> + 'a> Type<'a> for ListKind<EK> {
    const KIND: Kind = Kind::List;
    const ELEMENT_KIND: Option<Kind> = Some(EK::KIND);
    type EncodeType = ();
    type DecodeType = &'a mut dyn ListAppender<'a, EK>;
    type ReferencedType = EK::ReferencedType;

    fn encode<E: Encoder>(encoder: &mut E, value: Self::EncodeType) -> Result<(), EncodeError> {
        todo!()
    }

    fn decode<D: Decoder<'a>>(decoder: &mut D) -> Result<Self::DecodeType, DecodeError> {
        todo!()
    }
}

pub trait ListElementKind: Type {}
impl ListElementKind for I32Type {}
impl ListElementKind for StringType {}
impl<'a, S: StructCodec<'a> + 'a> ListElementKind for StructKind<S> {}

pub struct IntN<const N: usize>;
impl<const N: usize> Private for IntN<N> {}
impl<'a, const N: usize> Type<'a> for IntN<N> {
    const KIND: Kind = Kind::IntN;
    const SIZE_LIMIT: Option<u32> = Some(N);
    type ReferencedType = ();
    type EncodeType = [u8; N];
    type DecodeType = [u8; N];

    fn encode<E: Encoder>(encoder: &mut E, value: Self::EncodeType) -> Result<(), EncodeError> {
        todo!()
    }

    fn decode<D: Decoder<'a>>(decoder: &mut D) -> Result<Self::DecodeType, DecodeError> {
        todo!()
    }
}

pub struct UIntN<const N: u32>;

pub trait ReferenceType {
    const NAME: &'static str;
}

impl ReferenceType for () {
    const NAME: &'static str = "";
}