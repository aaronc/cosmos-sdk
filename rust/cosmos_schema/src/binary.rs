use crate::allocator::BorrowAllocator;
use crate::kind::ListElementKind;
use crate::list::ListAppender;
use crate::r#struct::StructCodec;
use crate::visitor::{DecodeError, Decoder};

// use crate::buffer::Buffer;
// use crate::kind::ListElementKind;
// use crate::list::{ListCodec};
// use crate::r#struct::StructCodec;
// use crate::value::Value;
// // pub fn decode_binary<'a, V: Value>(buf: &'a [u8], borrow_allocator: &'a mut BorrowAllocator) -> Result<(V::MaybeBorrowed<'a>, usize), DecodeError> {
// //
// // }
//
pub struct BinaryDecoder<'a> {
    buf: &'a [u8],
    borrow_allocator: &'a mut BorrowAllocator,
}

impl<'a> Decoder<'a> for BinaryDecoder<'a> {
    fn decode_i32(&mut self) -> Result<i32, DecodeError> {
        // let (bytes, rest) = self.buf.split_at(size_of::<i32>());
        // let value = i32::from_le_bytes(bytes.try_into().unwrap());
        // self.buf = rest;
        // Ok(value)
        todo!()
    }

    fn decode_u32(&mut self) -> Result<u32, DecodeError> {
        let (bytes, rest) = self.buf.split_at(size_of::<i32>());
        let value = u32::from_le_bytes(bytes.try_into().unwrap());
        self.buf = rest;
        Ok(value)
    }

    fn decode_str(&mut self) -> Result<&'a str, DecodeError> {
        // let len = self.read_u32()?;
        // let (bytes, rest) = self.buf.split_at(len as usize);
        // let value = std::str::from_utf8(bytes).map_err(|_| DecodeError::InvalidUtf8)?;
        // self.buf = rest;
        // Ok(value)
        todo!()
    }

    fn decode_struct<'b, V: StructCodec + 'a>(&'b mut self, v: &'b mut V) -> Result<(), DecodeError> {
        for (i, elem) in V::FIELDS.iter().enumerate() {
            v.decode_field(i, self)?;
        }
        Ok(())
    }

    fn decode_list<'b, EK: ListElementKind>(&'b mut self, v: &'b mut dyn ListAppender<'a, 'b, EK>) -> Result<(), DecodeError> {
        todo!()
    }

    // fn decode_list<V: ListCodec<'a, EK>, EK: ListElementKind>(&mut self, v: &'a mut V) -> Result<(), DecodeError> {
    //     let size = self.decode_u32()? as usize;
    //     let mut builder = V::new_builder(v, Some(size))?;
    //     for _ in 0..size {
    //         let elem = EK::decode(self)?;
    //         V::append(&mut builder, elem)?;
    //     }
    //     V::finish_building(builder)
    // }

    // fn read_struct<V: StructCodec<'a>>(&'a mut self) -> Result<() DecodeError> {
    //     // let len = self.read_u32();
    //     // let (mut buf, rest) = self.buf.split_at(len as usize);
    //     // self.buf = rest;
    //     // let mut nested = BinaryDecoder {
    //     //     buf,
    //     //     borrow_allocator: self.borrow_allocator,
    //     // };
    //     // V::decode(&mut nested)
    //     todo!()
    // }

    // fn decode_enum(&mut self) -> Result<i32, DecodeError> {
    //     self.decode_i32()
    // }
}
