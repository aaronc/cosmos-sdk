use crate::kind::{Kind, Type};
use crate::r#struct::{StructCodec, StructType};

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Field<'a> {
    pub name: &'a str,
    pub field_type: FieldType<'a>,
}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct FieldType<'a> {
    pub kind: Kind,
    pub nullable: bool,
    pub referenced_type: &'a str,
}

impl<'a> Field<'a> {
    pub const fn new(name: &'a str, field_type: FieldType) -> Self {
        Self {
            name,
            field_type,
        }
    }
}

pub fn to_field_type<'a, T: Type<'a>>() -> FieldType<'a> {
    FieldType {
        kind: T::KIND,
        nullable: T::NULLABLE,
        referenced_type: T::ReferencedType::NAME,
    }
}

// impl StructCodec for Field<'_> {
//     const NAME: &'static str = "Field";
//     const FIELDS: &'static [Field<'static>] = &[
//         Field {
//             name: "name",
//             kind: Kind::String,
//             nullable: false,
//             referenced_type: None,
//         },
//         Field {
//             name: "kind",
//             kind: Kind::Enum,
//             nullable: false,
//             referenced_type: Some("Kind"),
//         },
//         Field {
//             name: "nullable",
//             kind: Kind::Bool,
//             nullable: false,
//             referenced_type: None,
//         },
//         Field {
//             name: "referenced_type",
//             kind: Kind::String,
//             nullable: true,
//             referenced_type: None,
//         },
//     ];
//     const SEALED: bool = false;
//     fn get_field(&self, index: usize) -> Result<DynamicValue, ()> {
//         match index {
//             0 => Ok(DynamicValue::String(self.name)),
//             1 => Ok(DynamicValue::Enum(self.kind as i32)),
//             2 => Ok(DynamicValue::Bool(self.nullable)),
//             3 => Ok(DynamicValue::String(NameString::from(self.referenced_type.unwrap_or(""))),
//             _ => Err(DecodeError::NoSuchField(index)),
//         }
//     }
// }
//
