use alloc::borrow::Cow;
use std::iter::FusedIterator;

#[repr(C)]
pub struct Str<'a, const MAX: usize> (CowArray<'a, u8, MAX>);
pub struct Bytes<'a, const MAX: usize> (CowArray<'a, u8, MAX>);

pub struct Repeated<'a, T: 'a, const MAX: usize>(CowArray<'a, T, MAX>);

#[repr(C)]
enum CowArray<'a, T: 'a, const MAX: usize> {
    Borrowed(&'a [T]),
    Owned(Array<T, MAX>),
}

#[repr(C)]
struct Array<T, const MAX: usize> {
    len: u32,
    data: [T; MAX],
}

struct MsgSend<'a> {
    from: Bytes<'a, 32>,
    to: Bytes<'a, 32>,
    coins: Repeated<'a, crate::Coin<'a>, 16>,
}

struct Coin<'a> {
    denom: Str<'a, 32>,
    amount: u64,
}
