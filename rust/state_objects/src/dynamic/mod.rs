#[derive(Debug, Clone, PartialEq)]
pub enum DynamicValue {
    Null,
    Text(String),
    Bytes(Vec<u8>),
    Tuple(Vec<DynamicValue>),
    Struct(Vec<(String, DynamicValue)>),
}