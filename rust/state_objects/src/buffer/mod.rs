use cosmossdk_core::{Code, err};

pub trait Writer {
    fn write(&mut self, bytes: &[u8]) -> cosmossdk_core::Result<()>;
}

impl Writer for Vec<u8> {
    fn write(&mut self, bytes: &[u8]) -> cosmossdk_core::Result<()> {
        self.extend_from_slice(bytes);
        Ok(())
    }
}
