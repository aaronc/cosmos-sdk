use num_enum::{FromPrimitive, IntoPrimitive};
use crate::enum_type::{EnumCodec, EnumKind, EnumType, EnumValueDefinition};
use crate::list::{ListAppender, ListReader};
use crate::r#struct::{StructCodec, StructType};
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

pub trait Type: Private {
    const KIND: Kind;
    const NULLABLE: bool = false;
    const SIZE_LIMIT: Option<usize> = None;
    const ELEMENT_KIND: Option<Kind> = None;
    type ReferencedType;

    type GetType<'a>;
    type SetType<'a: 'b, 'b>;

    fn encode<'a, E: Encoder>(encoder: &mut E, value: Self::GetType<'a>) -> Result<(), EncodeError>;
    fn decode<'a: 'b, 'b, D: Decoder<'a> + 'a>(decoder: &'b mut D, set_value: Self::SetType<'a, 'b>) -> Result<(), DecodeError>;
}

impl<S> Private for StructKind<S> {}
impl<S: StructCodec> Type for StructKind<S>
where
        for<'a> S: 'a,
{
    const KIND: Kind = Kind::Struct;
    type ReferencedType = S;
    type GetType<'a> = &'a S;
    type SetType<'a: 'b, 'b> = &'b mut S
    where
        S: 'a;

    fn encode<'a, E: Encoder>(encoder: &mut E, value: Self::GetType<'a>) -> Result<(), EncodeError> {
        encoder.encode_struct(value)
    }

    fn decode<'a: 'b, 'b, D: Decoder<'a> + 'a>(decoder: &'b mut D, set_value: Self::SetType<'a, 'b>) -> Result<(), DecodeError> {
        decoder.decode_struct(set_value)
    }
}

impl<EK> Private for ListKind<EK> {}
impl<EK: ListElementKind + 'static> Type for ListKind<EK> {
    const KIND: Kind = Kind::List;
    const ELEMENT_KIND: Option<Kind> = Some(EK::KIND);
    type ReferencedType = EK::ReferencedType;
    type GetType<'a> = &'a dyn ListReader<'a, EK>;
    type SetType<'a: 'b, 'b> = &'b mut dyn ListAppender<'a, 'b, EK>;

    fn encode<'a, E: Encoder>(encoder: &mut E, value: Self::GetType<'a>) -> Result<(), EncodeError> {
        encoder.encode_list(value)
    }

    fn decode<'a: 'b, 'b, D: Decoder<'a> + 'a>(decoder: &'b mut D, set_value: Self::SetType<'a, 'b>) -> Result<(), DecodeError> {
        decoder.decode_list(set_value)
    }
}


impl Private for I32Type {}
impl Type for I32Type {
    const KIND: Kind = Kind::Int32;
    type ReferencedType = ();
    type GetType<'a> = &'a i32;
    type SetType<'a: 'b, 'b> = &'b mut i32;

    fn encode<'a, E: Encoder>(encoder: &mut E, value: Self::GetType<'a>) -> Result<(), EncodeError> {
        encoder.encode_i32(*value)
    }

    fn decode<'a: 'b, 'b, D: Decoder<'a> + 'a>(decoder: &'b mut D, set_value: Self::SetType<'a, 'b>) -> Result<(), DecodeError> {
        *set_value = decoder.decode_i32()?;
        Ok(())
    }
}

impl Private for StringType {}
impl Type for StringType {
    const KIND: Kind = Kind::String;
    type ReferencedType = ();
    type GetType<'a> = &'a str;
    type SetType<'a: 'b, 'b> = &'b mut &'a str;

    fn encode<'a, E: Encoder>(encoder: &mut E, value: Self::GetType<'a>) -> Result<(), EncodeError> {
        encoder.encode_str(value)
    }

    fn decode<'a: 'b, 'b, D: Decoder<'a> + 'a>(decoder: &'b mut D, set_value: Self::SetType<'a, 'b>) -> Result<(), DecodeError> {
        *set_value = decoder.decode_str()?;
        Ok(())
    }
}

pub struct NullableType<K> {
    _phantom: std::marker::PhantomData<K>,
}

// impl<K: Type> Private for NullableType<K> {}
// impl<K: Type> Type for NullableType<K> {
//     const KIND: Kind = K::KIND;
//     const NULLABLE: bool = true;
//     type ReferencedType = K::ReferencedType;
//     type GetType<'a> = Option<K::GetType<'a>>;
//     type SetType<'a> = Option<K::SetType<'a>>;
//
//     fn encode<E: Encoder>(encoder: &mut E, value: &Self::GetType<'_>) -> Result<(), EncodeError> {
//         todo!()
//     }
//
//     fn decode<'a, D: Decoder<'a>>(decoder: &mut D, set_value: &'a mut Self::SetType<'a>) -> Result<(), DecodeError> {
//         todo!()
//     }
// }

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

pub trait ListElementKind: Type {}
// impl ListElementKind for I32Type {}
// impl ListElementKind for StringType {}

// pub struct IntN<const N: usize>;
// impl<const N: usize> Private for IntN<N> {}
// impl<const N: usize> Type for IntN<N> {
//     const KIND: Kind = Kind::IntN;
//     const SIZE_LIMIT: Option<usize> = Some(N);
//     type ReferencedType = ();
//     type GetType<'a> = [u8; N];
//     type SetType<'a> = [u8; N];
//
//     fn encode<E: Encoder>(encoder: &mut E, value: &Self::GetType<'_>) -> Result<(), EncodeError> {
//         todo!()
//     }
//
//     fn decode<'a, D: Decoder<'a>>(decoder: &mut D, set_value: &'a mut Self::SetType<'a>) -> Result<(), DecodeError> {
//         todo!()
//     }
// }

pub struct UIntN<const N: u32>;

pub trait ReferenceTypeCodec {
    const NAME: &'static str;
}

impl ReferenceTypeCodec for () {
    const NAME: &'static str = "";
}

pub enum ReferenceType<'a> {
    Struct(StructType<'a>),
    Enum(EnumType<'a>),
}