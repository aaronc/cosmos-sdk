pub trait Buffer {
    fn write(&mut self, bytes: &[u8]) -> Result<(), Error>;
}

pub enum Error {
    OutOfSpace,
}

impl Buffer for Vec<u8> {
    // TODO maybe change this to write from back
    fn write(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.extend_from_slice(bytes);
        Ok(())
    }
}
