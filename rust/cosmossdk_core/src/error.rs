use core::fmt::{Debug, Display, Formatter};
use crate::Code;

pub struct Error(pub Code, pub String);

impl From<Code> for Error {
    fn from(value: Code) -> Self {
        Error(value, String::new())
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Error {:?}: {:?}", self.0, self.1)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Error {:?}: {:?}", self.0, self.1)
    }
}

#[cfg(not(feature = "no-std"))]
impl std::error::Error for Error {}