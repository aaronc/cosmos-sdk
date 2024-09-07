use crate::kind::Kind;

pub trait Value {
    type Borrowed<'a>;
    const KIND: Kind;
    const NULLABLE: bool = false;
}

impl Value for u8 {
    type Borrowed<'a> = u8;
    const KIND: Kind = Kind::Uint8;
}

impl Value for u16 {
    type Borrowed<'a> = u16;
    const KIND: Kind = Kind::Uint16;
}

impl Value for u32 {
    type Borrowed<'a> = u32;
    const KIND: Kind = Kind::Uint32;
}

impl Value for u64 {
    type Borrowed<'a> = u64;
    const KIND: Kind = Kind::Uint64;
}

impl Value for i8 {
    type Borrowed<'a> = i8;
    const KIND: Kind = Kind::Int8;
}

impl Value for i16 {
    type Borrowed<'a> = i16;
    const KIND: Kind = Kind::Int16;
}

impl Value for i32 {
    type Borrowed<'a> = i32;
    const KIND: Kind = Kind::Int32;
}

impl Value for i64 {
    type Borrowed<'a> = i64;
    const KIND: Kind = Kind::Int64;
}

impl Value for bool {
    type Borrowed<'a> = bool;
    const KIND: Kind = Kind::Bool;
}

impl Value for str {
    type Borrowed<'a> = &'a str;
    const KIND: Kind = Kind::String;
}

impl <V: Value> Value for Option<V> {
    type Borrowed<'a> = Option<V::Borrowed<'a>>;
    const KIND: Kind = V::KIND;
    const NULLABLE: bool = true;
}