#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Address {
    len: u8,
    bytes: [u8; 63],
}

impl Default for Address {
    fn default() -> Self {
        Self {
            len: 0,
            bytes: [0; 63],
        }
    }
}

