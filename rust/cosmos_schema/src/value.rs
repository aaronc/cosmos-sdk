use crate::kind::{I32Type, Kind, NullableType, StringType, Type};
use crate::visitor::{Decoder, DecodeError, Encoder, EncodeError};

pub trait Value<'a, K: Type>
where
    <K as Type>::SetType<'a>: 'a,
{
    fn to_encode_value(&'a self) -> &K::GetType<'a>;
    fn decode<'a, D: Decoder<'a>>(&'a mut self, decoder: &mut D) -> Result<(), DecodeError>;
}

impl Value<'_, I32Type> for i32 {
    fn to_encode_value(&'_ self) -> &<I32Type as Type>::GetType<'_> {
        self
    }

    fn decode<'a, D: Decoder<'a>>(&'a mut self, decoder: &mut D) -> Result<(), DecodeError> {
        *self = decoder.decode_i32()?;
        Ok(())
    }
}

impl<'a> Value<'a, StringType> for &'a str {
    fn to_encode_value(&'a self) -> &<StringType as Type>::GetType<'_> {
        self
    }

    fn decode<'a, D: Decoder<'a>>(&'a mut self, decoder: &mut D) -> Result<(), DecodeError> {
        *self = decoder.decode_str()?;
        Ok(())
    }
}

#[cfg(not(feature = "no_std"))]
impl<'a> Value<'a, StringType> for String {
    fn to_encode_value(&'a self) -> &<StringType as Type>::GetType<'_> {
        &self.as_str()
    }

    fn decode<'a, D: Decoder<'a>>(&'a mut self, decoder: &mut D) -> Result<(), DecodeError> {
        *self = decoder.decode_str()?.to_owned();
        Ok(())
    }
}

impl<'a, K: Type, V: Value<'a, K> + Sized> Value<'a, NullableType<K>> for Option<V> {
    fn to_encode_value(&'a self) -> &<NullableType<K> as Type>::GetType<'_> {
        self
    }

    fn decode<'a, D: Decoder<'a>>(&'a mut self, decoder: &mut D) -> Result<(), DecodeError> {
        todo!()
    }
}