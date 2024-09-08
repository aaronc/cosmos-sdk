mod simple;

use crate::allocator::BorrowAllocator;
use crate::buffer::Buffer;
use crate::errors::{DecodeError, EncodeError};
use crate::value::Value;

pub unsafe trait ValueCodec: Value {
    fn encode<B: Buffer>(buf: &mut B, value: Self::Borrowed) -> Result<(), EncodeError>;

    fn decode<'a>(buf: &'a [u8], borrow_allocator: &'a mut BorrowAllocator) -> Result<(Self::Borrowed<'a>, usize), DecodeError>;

    fn size_hint(key: &Self::Borrowed) -> Option<usize> { None }
}
