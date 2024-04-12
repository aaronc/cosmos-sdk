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

    codespace: String,

    #[cfg(not(feature = "no_alloc"))]
    message: String,

    #[cfg(feature = "no_alloc")]
    message: fixed_str::FixedString,
}

impl Error {
    pub fn new(code: Code, file: &str, line: u32) -> Self {
        #[cfg(not(feature = "no_alloc"))]
            let mut message = String::new();
        #[cfg(feature = "no_alloc")]
            let mut message = fixed_str::FixedString::new();
        let _ = core::fmt::write(&mut message, format_args!("{}:{}", file, line));
        Self {
            code,
            message,
            codespace: "".into(),
        }
    }

    pub fn new_fmt(code: Code, file: &str, line: u32, args: core::fmt::Arguments<'_>) -> Self {
        #[cfg(not(feature = "no_alloc"))]
            let mut message = String::new();
        #[cfg(feature = "no_alloc")]
            let mut message = fixed_str::FixedString::new();
        let _ = core::fmt::write(&mut message, format_args!("{}:{}: ", file, line));
        let _ = core::fmt::write(&mut message, args);
        Self {
            code,
            message,
            codespace: "".into(),
        }
    }
    pub fn with_codespace(mut self, codespace: &str) -> Self {
        self.codespace = codespace.into();
        self
    }

    fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error")?;
        if self.code != Code::Other {
            write!(f, ":{:?}", self.code)?
        }
        if !self.codespace.is_empty() {
            write!(f, ":{}", self.codespace)?
        }
        if !self.message.is_empty() {
            write!(f, ": {}", self.message)?
        }
        Ok(())
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
        self.format(f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(f)
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
macro_rules! fmt_error {
    ($code:ident) => {
        $crate::Error::new($crate::Code::$code, file!(), line!())
    };
    ($code:ident, $($arg:tt)*) => {
        $crate::Error::new_fmt($crate::Code::$code, file!(), line!(), core::format_args!($($arg)*))
    };
    ($($arg:tt)*) => {
        $crate::Error::new_fmt($crate::Code::Other, file!(), line!(), core::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {
        return Err($crate::fmt_error!($($arg)*));
    };
}

#[macro_export]
macro_rules! ensure {
    ($cond:expr, $($arg:tt)*) => {
        if !$cond {
            return Err($crate::fmt_error!($($arg)*));
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_code_fmt() {
        let err = fmt_error!(InvalidArgument, "expected foo");
        assert_eq!(format!("{:?}", err), "Error:InvalidArgument: cosmos_result/src/lib.rs:148: expected foo");
    }

    #[test]
    fn test_code() {
        let err = fmt_error!(InvalidArgument);
        assert_eq!(format!("{:?}", err), "Error:InvalidArgument: cosmos_result/src/lib.rs:154");
    }

    #[test]
    fn test_fmt() {
        let err = fmt_error!("some error: {}", 1);
        assert_eq!(format!("{:?}", err), "Error: cosmos_result/src/lib.rs:160: some error: 1");
    }
}