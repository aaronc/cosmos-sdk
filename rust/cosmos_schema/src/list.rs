use std::thread::Builder;
use crate::kind::{ListElementKind, ListKind, TypeLevelKind};
use crate::value::Value;
use crate::visitor::DecodeError;

pub trait ListCodec<'a, EK: ListElementKind<'a>>: Sized {
    type Builder;
    fn new_builder(&'a mut self, size_hint: Option<usize>) -> Result<Self::Builder, DecodeError>;
    fn append(builder: &mut Self::Builder, value: EK::DecodeType) -> Result<(), DecodeError>;
    fn finish_building(builder: Self::Builder) -> Result<(), DecodeError>;
}

impl<'a, EK: ListElementKind<'a>, V: Value<'a, EK> + 'a> ListCodec<'a, EK> for Vec<V> {
    type Builder = VecBuilder<'a, V>;

    fn new_builder(&'a mut self, size_hint: Option<usize>) -> Result<Self::Builder, DecodeError> {
        *self = Vec::with_capacity(size_hint.unwrap_or(0));
        Ok(VecBuilder {
            target: self,
        })
    }

    fn append(builder: &mut Self::Builder, value: EK::DecodeType) -> Result<(), DecodeError> {
        builder.target.push(V::from_decode_value(value));
        Ok(())
    }

    fn finish_building(builder: Self::Builder) -> Result<(), DecodeError> {
        Ok(())
    }
}

pub struct VecBuilder<'a, V> {
    target: &'a mut Vec<V>,
}

impl<'a, EK: ListElementKind<'a>, L: ListCodec<'a, EK> + Sized + 'a> Value<'a, ListKind<L, EK>> for L {
    fn to_encode_value(&'a self) -> <ListKind<L, EK> as TypeLevelKind<'a>>::EncodeType {
        self
    }

    fn from_decode_value(value: <ListKind<L, EK> as TypeLevelKind<'a>>::DecodeType) -> Self {
        value
    }
}

pub trait ListAppender<'a, E: ListElementKind<'a>> {
    fn append(&'a mut self) -> &'a mut E;
}
