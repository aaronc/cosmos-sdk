extern crate core;

use crate::Code;
use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub fn ok<T:Default>() -> Result<T> {
    Ok(Default::default())
}

pub fn err<T>(code: Code, msg: String) -> Result<T> {
    Err(Error(code, msg))
}

#[macro_export]
macro_rules! err {
    ($code:expr) => {
        crate::err($code, "".to_string())
    };
    ($code:expr, $msg:expr) => {
        crate::err($code, $msg.to_string())
    };
    ($code:expr, $msg:expr, $($args:expr),*) => {
        crate::err($code, format!($msg, $($args),*))
    };
}
