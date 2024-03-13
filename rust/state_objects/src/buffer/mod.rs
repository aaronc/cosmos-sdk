use cosmossdk_core::{Code, err};

pub trait Writer {
    fn write(&mut self, bytes: &[u8]) -> cosmossdk_core::Result<()>;
}

pub trait Reader {
    fn read(&mut self, n: usize) -> cosmossdk_core::Result<&[u8]>;
    fn read_all(&mut self) -> cosmossdk_core::Result<&[u8]>;
}

impl Writer for Vec<u8> {
    fn write(&mut self, bytes: &[u8]) -> cosmossdk_core::Result<()> {
        self.extend_from_slice(bytes);
        Ok(())
    }
}

pub struct BytesReader<'a>(&'a [u8]);

impl <'a> BytesReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self(bytes)
    }
}

impl <'a> From<&'a [u8]> for BytesReader<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Self(bytes)
    }
}

impl <'a> Reader for BytesReader<'a> {
    fn read(&mut self, n: usize) -> cosmossdk_core::Result<&[u8]> {
        if self.0.len() < n {
            return err!(Code::OutOfRange)
        }
        let (bytes, rest) = self.0.split_at(n);
        self.0 = rest;
        Ok(bytes)
    }

    fn read_all(&mut self) -> cosmossdk_core::Result<&[u8]> {
        self.0 = &[];
        Ok(self.0)
    }
}
