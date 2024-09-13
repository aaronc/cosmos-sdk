use crate::kind::{I32Type, Kind, NullableType, StringType, Type};
use crate::visitor::{Decoder, DecodeError, Encoder, EncodeError};

pub trait Value<'a, K: Type>
where
    Self: 'a,
{
    fn to_encode_value<'b>(&'b self) -> K::GetType<'b>;
    fn decode<'b, D: Decoder<'a>>(&'b mut self, decoder: &mut D) -> Result<(), DecodeError>;
    // TODO: ideally values don't need to interact directly with the decoder and we can use some sort of closure, but I couldn't get it to work yet:
    // fn decode<'b, F: FnOnce(K::SetType<'a, 'b>) -> Result<(), DecodeError>>(&'b mut self, f: F) -> Result<(), DecodeError>
    // where
    //     F: 'b;
}

// impl Value<I32Type> for i32 {
//     // fn to_encode_value(&'_ self) -> <I32Type as Type>::GetType<'_> {
//     //     *self
//     // }
//     //
//     // fn decode<D: Decoder<'a>>(&'a mut self, decoder: &mut D) -> Result<(), DecodeError> {
//     //     *self = decoder.decode_i32()?;
//     //     Ok(())
//     // }
// }
//
impl<'a> Value<'a, StringType> for &'a str {
    fn to_encode_value<'b>(&'b self) -> <StringType as Type>::GetType<'b> {
        *self
    }

    fn decode<'b, D: Decoder<'a>>(&'b mut self, decoder: &mut D) -> Result<(), DecodeError> {
        *self = decoder.decode_str()?;
        Ok(())
    }

    // fn decode<'b, F: FnOnce(<StringType as Type>::SetType<'a, 'b>) -> Result<(), DecodeError>>(&'b mut self, f: F) -> Result<(), DecodeError> {
    //     f(self)
    // }
    //
    // fn decode<'a, D: Decoder<'a>>(&'a mut self, decoder: &mut D) -> Result<(), DecodeError> {
    //     *self = decoder.decode_str()?;
    //     Ok(())
    // }
}
//
#[cfg(not(feature = "no_std"))]
impl<'a> Value<'a, StringType> for String {
    fn to_encode_value<'b>(&'b self) -> <StringType as Type>::GetType<'b> {
        self.as_str()
    }

    fn decode<'b, D: Decoder<'a>>(&'b mut self, decoder: &mut D) -> Result<(), DecodeError> {
        *self = decoder.decode_str()?.to_owned();
        Ok(())
    }

    // fn decode<'b, F: FnOnce(<StringType as Type>::SetType<'a, 'b>) -> Result<(), DecodeError>>(&'b mut self, f: F) -> Result<(), DecodeError> {
    //     let mut s = "";
    //     f(&mut s)?;
    //     *self = s.to_owned();
    //     Ok(())
    // }
    // fn to_encode_value(&self) -> <StringType as Type>::GetType {
    //     self.as_str()
    // }
    //
    // fn decode<D: Decoder<'a>>(&'a mut self, decoder: &mut D) -> Result<(), DecodeError> {
    //     *self = decoder.decode_str()?.to_owned();
    //     Ok(())
    // }
}

// impl<'a, K: Type, V: Value<'a, K> + Sized> Value<'a, NullableType<K>> for Option<V> where <K as Type>::SetType<'a>: 'a {
//     fn to_encode_value(&'a self) -> <NullableType<K> as Type>::GetType<'a> {
//         // self
//         todo!()
//     }
//
//     fn decode<D: Decoder<'a>>(&'a mut self, decoder: &mut D) -> Result<(), DecodeError> {
//         todo!()
//     }
// }