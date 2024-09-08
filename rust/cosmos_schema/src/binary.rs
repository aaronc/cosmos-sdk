use crate::allocator::BorrowAllocator;
use crate::buffer::Buffer;
use crate::errors::DecodeError;
use crate::value::Value;

pub fn decode_binary<'a, V: Value>(buf: &'a [u8], borrow_allocator: &'a mut BorrowAllocator) -> Result<(V::Borrowed<'a>, usize), DecodeError> {

}
