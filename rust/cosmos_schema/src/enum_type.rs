use crate::kind::Kind;
use crate::name::NameString;

#[non_exhaustive]
pub struct EnumType {
    pub name: NameString,
    pub values: Vec<EnumValueDefinition>,
    pub numeric_kind: Kind,
}

#[non_exhaustive]
pub struct EnumValueDefinition {
    pub name: NameString,
    pub value: i32,
}