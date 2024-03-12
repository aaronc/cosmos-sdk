use core::borrow::Borrow;
use crate::{Code, err};
use crate::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum AgentId {
    Module(ModuleId),
    Account(Address),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Address {
    len: u8,
    data: [u8; 256],
}

impl Default for Address {
    fn default() -> Self {
        Address { len: 0, data: [0; 256] }
    }
}

impl Address {
    fn new(s: &[u8]) -> crate::Result<Self> {
        let len = s.len();
        if len > 256 {
            return err!(Code::InvalidArgument, "address can be at most 256 bytes, received {} byte", len);
        }
        let mut data = [0; 256];
        data[0..len].copy_from_slice(s);
        Ok(Address { len: len as u8, data })
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

#[derive(Debug, Clone, PartialEq, Eq)]
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