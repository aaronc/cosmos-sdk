use crate::buffer::Buffer;
use crate::errors::{DecodeError, EncodeError};
use crate::value::Value;
use crate::value_codec::{ValueCodec};

pub trait StateObjectKeyCodec {
        type In<'a>;
        type Out<'a>;
        type Keys<'a>;

        fn encode<B: Buffer>(buf: &mut B, key: Self::In<'_>) -> Result<(), EncodeError>;

        fn decode<B: Buffer>(buf: &B) -> Result<Self::Out<'_>, DecodeError>;
}

pub trait KeyCodec: ValueCodec {
    fn encode<B: Buffer>(buf: &mut B, key: Self::T::Borrowed<'_>) -> Result<(), EncodeError>;

    fn decode<'a>(buf: &'a [u8]) -> Result<(Self::T::Borrowed<'a>, usize), DecodeError>;

    fn encode_terminal<B: Buffer>(buf: &mut B, key: Self::T::Borrowed<'_>) -> Result<(), EncodeError> {
        Self::encode(buf, key)
    }

    fn decode_terminal<B: Buffer>(buf: &mut B, key: Self::T::Borrowed<'_>) -> Result<(), EncodeError> {
        Self::encode(buf, key)
    }

    fn size_hint(key: Self::Borrowed<'_>) -> usize;
}