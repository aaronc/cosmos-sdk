use core::marker::PhantomData;
use core::{borrow::Borrow, fmt::Write, str::from_utf8_unchecked};
use core::fmt::Display;

use crate::bytes::{Bytes, BytesWriter};
use crate::result::RawResult;
use crate::zerocopy::ZeroCopy;

#[repr(C)]
pub struct Str {
    pub(crate) ptr: Bytes,
    _phantom: PhantomData<str>,
}

unsafe impl ZeroCopy for Str {}

impl Str {
    pub fn set(&mut self, content: &str) -> RawResult<()> {
        self.ptr.set(content.as_bytes())
    }

    pub fn new_writer(&mut self) -> RawResult<StrWriter> {
        self.ptr.new_writer().map(|bz| StrWriter { bz })
    }

    pub fn as_str(&self) -> &str {
        unsafe { from_utf8_unchecked(self.ptr.borrow()) }
    }
}

impl<'a> Borrow<str> for Str {
    fn borrow(&self) -> &str {
        unsafe { from_utf8_unchecked(self.ptr.borrow()) }
    }
}

impl Display for Str {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.borrow())
    }
}

pub struct StrWriter<'a> {
    bz: BytesWriter<'a>,
}

impl<'a> Write for StrWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.bz.write(s.as_bytes()).map_err(|_| core::fmt::Error)
    }
}
