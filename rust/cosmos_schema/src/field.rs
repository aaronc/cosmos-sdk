use crate::kind::Kind;
use crate::name::NameString;

#[non_exhaustive]
pub struct Field<'a> {
    pub name: &'a str,
    pub kind: Kind,
    pub nullable: bool,
    pub referenced_type: Option<&'a str>,
}

