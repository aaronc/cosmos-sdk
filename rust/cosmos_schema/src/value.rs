use crate::kind::{I32Type, Kind, NullableType, StringType, Type};
use crate::visitor::{Decoder, DecodeError, Encoder, EncodeError};

pub trait Value<'a, K: Type>
where
    <K as Type>::SetType<'a>: 'a,
{
    fn to_encode_value(&'a self) -> &K::GetType<'a>;
    fn decode<R, D: FnOnce(&'a mut K::SetType<'a>) -> R>(&'a mut self, decoder: &D) -> R;
}

impl Value<'_, I32Type> for i32 {
    fn to_encode_value(&'_ self) -> &<I32Type as Type>::GetType<'_> {
        self
    }

    fn decode<R, D: FnOnce(&'_ mut <I32Type as Type>::SetType<'_>) -> R>(&'_ mut self, decoder: &D) -> R {
        decoder(self)
    }
}

impl<'a> Value<'a, StringType> for &'a str {
    fn to_encode_value(&'a self) -> &<StringType as Type>::GetType<'_> {
        self
    }

    fn decode<R, D: FnOnce(&'a mut <StringType as Type>::SetType<'a>) -> R>(&'a mut self, decoder: &D) -> R {
        decoder(self)
    }
}

#[cfg(not(feature = "no_std"))]
impl<'a> Value<'a, StringType> for String {
    fn to_encode_value(&'a self) -> &<StringType as Type>::GetType<'_> {
        &self.as_str()
    }

    fn decode<R, D: FnOnce(&'a mut <StringType as Type>::SetType<'a>) -> R>(&'a mut self, decoder: &D) -> R {
        let mut s = "";
        let res = decoder(&mut s);
        *self = s.to_string();
        res
    }
}

impl<'a, K: Type, V: Value<'a, K> + Sized> Value<'a, NullableType<K>> for Option<V> {
    fn to_encode_value(&'a self) -> &<NullableType<K> as Type>::GetType<'_> {
        self
    }

    fn decode<R, D: FnOnce(&'a mut NullableType<K>::SetType<'a>) -> R>(&'a mut self, decoder: &D) -> R {
        todo!()
    }
}