mod code;
#[cfg(feature = "no_alloc")]
mod fixed_str;

use std::fmt::{Debug, Display, Formatter};
pub use code::*;

pub type Result<T> = core::result::Result<T, Error>;

pub fn ok<T: Default>() -> Result<T> {
    Ok(Default::default())
}

pub struct Error {
    code: Code,

    #[cfg(not(feature = "no_alloc"))]
    message: String,

    #[cfg(feature = "no_alloc")]
    message: fixed_str::FixedString,
}

impl Error {
    pub fn new(code: Code) -> Self {
        Self {
            code,
            #[cfg(not(feature = "no_alloc"))]
            message: String::new(),
        }
    }

    pub fn new_fmt(code: Code, args: core::fmt::Arguments<'_>) -> Self {
        #[cfg(not(feature = "no_alloc"))]
            let mut msg = String::new();
        #[cfg(feature = "no_alloc")]
            let mut msg = fixed_str::FixedString::new();
        let _ = core::fmt::write(&mut msg, args);
        Self {
            code,
            message: msg,
        }
    }
}

// impl core::fmt::Write for Error {
//     fn write_str(&mut self, s: &str) -> core::fmt::Result {
//         #[cfg(not(feature = "no_alloc"))]
//         self.message.push_str(s);
//         #[cfg(feature = "no_alloc")]
//         let _ = self.message.push_str(s);
//         Ok(())
//     }
// }

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error {:?}: {}", self.code, self.message)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error {:?}: {}", self.code, self.message)
    }
}


#[cfg(not(feature = "no_std"))]
impl std::error::Error for Error {}

#[cfg(all(feature = "no_std", feature = "error_in_core"))]
impl core::error::Error for Error {}

// #[cfg(not(feature = "no_std"))]
// impl <E> From<E> for Error
//     where E: std::error::Error + Send + Sync + 'static,
// {
//
//     fn from(value: E) -> Self {
//         Error {
//             code: Code::Unknown,
//             message: value.to_string() // TODO use value.fmt
//         }
//     }
// }

#[macro_export]
macro_rules! new_error {
    ($code:expr) => {
        $crate::Error::new($code)
    };
    ($code:expr, $($arg:tt)*) => {
        $crate::Error::new_fmt($code, core::format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! bail {
    ($code:expr) => {
        return core::result::Result::Err($crate::Error::new($code));
    };
    ($code:expr, $($arg:tt)*) => {
        return core::result::Result::Err($crate::new_error!($code, $($arg)*));
    };
}

#[macro_export]
macro_rules! ensure {
    ($cond:expr, $code:expr, $($arg:tt)*) => {
        if !($cond) {
            $crate::bail!($code, $($arg)*);
        }
    };
}

