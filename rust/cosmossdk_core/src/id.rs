use core::borrow::Borrow;
use crate::{Code, err};
use crate::error::Error;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum AgentId {
    #[default]
    Unknown,
    Module(ModuleId),
    Account(Address),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Address {
    len: u8,
    data: [u8; 255],
}

impl Address {
    pub fn new(s: &[u8]) -> crate::Result<Self> {
        let len = s.len();
        if len > 255 {
            return err!(Code::InvalidArgument, "address can be at most 256 bytes, received {} byte", len);
        }
        let mut addr = Address::default();
        addr.data[0..len].copy_from_slice(s);
        addr.len = len as u8;
        Ok(addr)
    }
}

impl Borrow<[u8]> for Address {
    fn borrow(&self) -> &[u8] {
        &self.data[0..self.len as usize]
    }
}

impl Address {
    pub fn as_slice(&self) -> &[u8] {
        &self.data[0..self.len as usize]
    }
}

impl Into<Vec<u8>> for Address {
    fn into(self) -> Vec<u8> {
        unsafe {
            Vec::from_raw_parts(self.data.to_vec().as_mut_ptr(), self.len as usize, 256)
        }
    }
}

impl Into<Vec<u8>> for &Address {
    fn into(self) -> Vec<u8> {
        self.as_slice().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ModuleId(Address);

impl ModuleId {
    pub fn new(id: &str) -> crate::Result<Self> {
        Ok(ModuleId(Address::new(id.as_bytes())?))
    }
}

impl Borrow<str> for ModuleId {
    fn borrow(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(self.0.borrow()) }
    }
}