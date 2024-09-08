use num_enum::{FromPrimitive, IntoPrimitive};
use crate::enum_type::{EnumCodec, EnumKind, EnumType, EnumValueDefinition};

#[non_exhaustive]
#[repr(u32)]
#[derive(FromPrimitive, IntoPrimitive, Debug, Clone, Copy, PartialEq, Eq, Default)]
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
    #[num_enum(catch_all)]
    #[default]
    Unknown(u32)
}

unsafe impl EnumCodec for Kind {
    const NAME: &'static str = "Kind";
    const VALUES: &'static [EnumValueDefinition<'static>] = &[
        EnumValueDefinition::new("String", Kind::String.into()),
        EnumValueDefinition::new("Bytes", Kind::Bytes.into()),
        EnumValueDefinition::new("Int8", Kind::Int8.into()),
        EnumValueDefinition::new("Uint8", Kind::Uint8.into()),
        EnumValueDefinition::new("Int16", Kind::Int16.into()),
        EnumValueDefinition::new("Uint16", Kind::Uint16.into()),
        // TODO
    ];
}