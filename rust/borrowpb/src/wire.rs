use num_enum::{IntoPrimitive, TryFromPrimitive};
use crate::buf::BytesWriter;

pub type Number = i32;

#[derive(PartialEq, Eq, Clone, Copy, Debug, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum WireType {
    /// int32, int64, uint32, uint64, sint32, sint64, bool, enum
    Varint = 0,
    /// fixed64, sfixed64, double
    Fixed64 = 1,
    /// string, bytes, embedded messages, packed repeated fields
    LengthDelimited = 2,
    /// Groups are not supported
    StartGroup = 3,
    /// Groups are not supported
    EndGroup = 4,
    /// fixed32, sfixed32, float
    Fixed32 = 5,
}

pub fn encode_tag(num: Number, wire_type: WireType) -> u64 {
    let wt: u8 = wire_type.into();
    ((num as u64) << 3) | (wt as u64)
}

pub fn decode_tag(tag: u64) -> (Number, WireType) {
    // TODO: handle error conditions
    let num = tag >> 3;
    let wt = tag & 0b111;
    (num as Number, WireType::try_from(wt as u8).unwrap())
}

// pub fn size_varint(v: u64) -> usize {
//     let mut size = 0;
//     let mut v = v;
//     loop {
//         size += 1;
//         v >>= 7;
//         if v == 0 {
//             break;
//         }
//     }
//     size
// }
//
// pub fn write_varint(v: u64, buf: &mut BytesWriter) {
//     let mut size = size_varint(v);
//     let mut bz = buf.write(size);
//     match size {
//         1 => {
//             bz[0] = v as u8;
//         }
//         2 => {
//             bz[0] = (v & 0xf | 0x80) as u8;
//             bz[1] = (v >> 7) as u8;
//         }
//     }
// }