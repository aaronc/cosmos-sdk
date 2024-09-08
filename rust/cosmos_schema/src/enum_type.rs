use crate::field::Field;
use crate::kind::Kind;
use crate::r#struct::{StructCodec, StructType};
use crate::value::Value;
use crate::visitor::{Decoder, DecodeError, Encoder, EncodeError};

#[non_exhaustive]
pub struct EnumType<'a> {
    pub name: &'a str,
    pub values: &'a [EnumValueDefinition<'a>],
    pub numeric_kind: Kind,
    pub sealed: bool,
}

// TODO: can we be more restrictive here but still have the type serialized as Kind
pub enum EnumKind {
    Int8 = 3,
    Uint8 = 4,
    Int16 = 5,
    Uint16 = 6,
    Int32 = 7,
}

#[non_exhaustive]
pub struct EnumValueDefinition<'a> {
    pub name: &'a str,
    pub value: i32,
}

impl<'a> EnumValueDefinition<'a> {
    pub const fn new(name: &'a str, value: i32) -> Self {
        Self {
            name,
            value,
        }
    }
}

pub unsafe trait EnumCodec {
    const NAME: &'static str;
    const VALUES: &'static [EnumValueDefinition<'static>];
}

unsafe impl StructCodec for EnumValueDefinition<'_> {
    const NAME: &'static str = "EnumValueDefinition";
    const FIELDS: &'static [Field<'static>] = &[
        Field::new("name", Kind::String, false, None),
        Field::new("value", Kind::Int32, false, None),
    ];
    const SEALED: bool = true;
    type MaybeBorrowed<'a> = EnumValueDefinition<'a>;

    fn encode_field<'a, V: Encoder<'a>>(value: Self::MaybeBorrowed<'a>, index: usize, visitor: &'a mut V) -> Result<(), EncodeError> {
        match index {
            0 => visitor.visit_str(value.name),
            1 => visitor.visit_i32(value.value),
            _ => Err(EncodeError::InvalidFieldIndex { index }),
        }
    }

    fn decode<'a, V: Decoder<'a>>(visitor: &'a mut V) -> Result<Self::MaybeBorrowed<'a>, DecodeError> {
        let name = visitor.read_str()?;
        let value = visitor.read_i32()?;
        Ok(EnumValueDefinition { name, value })
    }


    // fn get_field(value: Self::Borrowed, index: usize) -> Result<DynamicValue, ()> {
    //     match index {
    //         0 => Ok(Value::to_dynamic(value.name)),
    //         1 => Ok(Value::to_dynamic(value.value)),
    //         _ => Err(()),
    //     }
    // }
    //
    // fn from_fields<'a>(fields: &'a [DynamicValue]) -> Result<Self::Borrowed<'a>, DecodeError> {
    //     if fields.len() != Self::FIELDS.len() {
    //         return Err(DecodeError::WrongNumberFields { expected: 2, got: fields.len() });
    //     }
    //     Ok(EnumValueDefinition {
    //         name: str::from_dynamic(&fields[0])?,
    //         value: i32::from_dynamic(&fields[1])?,
    //     })
    // }
}

impl<E: EnumCodec + TryFrom<i32>> Value for E {
    type MaybeBorrowed<'a> = E;
    const KIND: Kind = Kind::Enum;
    const NULLABLE: bool = false;

    fn encode<'a, V: Encoder<'a>>(value: Self::MaybeBorrowed<'a>, visitor: &'a V) {
        visitor.visit_enum(value.into());
    }

    // fn to_dynamic(value: Self::Borrowed) -> DynamicValue {
    //     DynamicValue::Enum(value.into())
    // }
    //
    // fn from_dynamic<'a>(value: &'a DynamicValue<'a>) -> Result<Self::Borrowed<'a>, DecodeError> {
    //     match value {
    //         DynamicValue::Enum(value) => Ok(E::try_from(*value).map_err(|_| DecodeError::InvalidEnumValue { value: *value })?),
    //         _ => Err(DecodeError::InvalidKind { expected: Kind::Enum, got: value.kind() }),
    //     }
    // }
}