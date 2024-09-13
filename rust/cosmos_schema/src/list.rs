use std::thread::Builder;
use crate::kind::{ListElementKind, ListKind, Type};
use crate::value::Value;
use crate::visitor::DecodeError;

// impl<'a, EK: ListElementKind, V: Value<'a, EK> + 'a> ListCodec<'a, EK> for Vec<V> {
//     type Builder = VecBuilder<'a, V>;
//
//     fn new_builder(&'a mut self, size_hint: Option<usize>) -> Result<Self::Builder, DecodeError> {
//         *self = Vec::with_capacity(size_hint.unwrap_or(0));
//         Ok(VecBuilder {
//             target: self,
//         })
//     }
//
//     fn append(builder: &mut Self::Builder, value: EK::DecodeType<'a>) -> Result<(), DecodeError> {
//         builder.target.push(V::from_decode_value(value));
//         Ok(())
//     }
//
//     fn finish_building(builder: Self::Builder) -> Result<(), DecodeError> {
//         Ok(())
//     }
// }
//
// pub struct VecBuilder<'a, V> {
//     target: &'a mut Vec<V>,
// }
//
// impl<'a, EK: ListElementKind<'a>, L: ListCodec<'a, EK> + Sized + 'a> Value<'a, ListKind<EK>> for L {
//     fn to_encode_value(&'a self) -> <ListKind<EK> as Type>::EncodeType<'a> {
//         self
//     }
//
//     fn from_decode_value(value: <ListKind<EK> as Type>::DecodeType<'a>) -> Self {
//         value
//     }
// }

pub trait ListAppender<'a: 'b, 'b, E: ListElementKind>
where
    E::SetType<'a, 'b>: 'a,
{
    fn append(&'b mut self) -> &mut E::SetType<'a, 'b>;
}

//
pub trait ListReader<'a: 'b, 'b, E: ListElementKind>: Iterator<Item=E::GetType<'a, 'b>>
where
    E::GetType<'a, 'b>: 'a,
{
    fn size_hint(&self) -> Option<usize>;
}
