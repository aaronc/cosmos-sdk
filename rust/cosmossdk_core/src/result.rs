extern crate core;

use crate::Code;
use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub fn ok<T:Default>() -> Result<T> {
    Ok(Default::default())
}

#[macro_export]
macro_rules! err {
    ($code:expr) => {
        core::result::Result::Err(crate::error::Error($code, "".to_string()))
    };
    ($code:expr, $msg:expr) => {
        core::result::Result::Err(crate::error::Error($code, $msg.to_string()))
    };
    ($code:expr, $msg:expr, $($args:expr),*) => {
        core::result::Result::Err(crate::error::Error($code, format!($msg, $($args),*)))
    };
}
