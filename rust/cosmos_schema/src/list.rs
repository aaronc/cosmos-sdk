use crate::kind::{ListKind, TypeLevelKind};
use crate::value::Value;
use crate::visitor::DecodeError;

pub trait List<'a, EK: TypeLevelKind<'a>> {
    // type Decoder: ListDecoder<'a, EK, E, Self>;
    fn init(size_hint: Option<usize>) -> Self;
    fn append(&mut self, value: EK::DecodeType) -> Result<(), DecodeError>;
}

impl <'a, EK: TypeLevelKind<'a>, V: Value<'a, EK>> List<'a, EK> for Vec<V> {
    fn init(size_hint: Option<usize>) -> Self {
        match size_hint {
            Some(size) => Vec::with_capacity(size),
            None => Vec::new(),
        }
    }

    fn append(&mut self, value: EK::DecodeType) -> Result<(), DecodeError> {
        self.push(V::from_decode_value(value));
        Ok(())
    }
}

// impl<'a, EK: TypeLevelKind<'a>, L: List<'a, EK>> Value<'a, ListKind<'a, EK, L>> for L {
//     fn to_encode_value(&'a self) -> &'a L { self }
//     fn from_decode_value(value: L) -> Self { value }
// }

// pub trait ListDecoder<'a, K: TypeLevelKind<'a>, E: Value<'a, K>, L: Value<'a, ListKind<'a, E>>> {
// }