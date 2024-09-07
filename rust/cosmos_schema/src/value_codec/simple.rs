use crate::allocator::BorrowAllocator;
use crate::buffer::Buffer;
use crate::errors::{DecodeError, EncodeError};
use crate::value_codec::ValueCodec;

impl ValueCodec for i32 {
    fn encode<B: Buffer>(buf: &mut B, value: Self::Borrowed) -> Result<(), EncodeError> {
        buf.write(&value.to_le_bytes()).map_err(EncodeError::BufferError)
    }

    fn decode(buf: &[u8], _: BorrowAllocator) -> Result<(Self::Borrowed, usize), DecodeError> {
        let (bytes, rest) = buf.split_at(size_of::<Self>());
        let value = i32::from_le_bytes(bytes.try_into().unwrap());
        Ok((value, rest.len()))
    }

    fn size_hint(key: &Self::Borrowed) -> Option<usize> {
        Some(size_of::<Self>())
    }
}