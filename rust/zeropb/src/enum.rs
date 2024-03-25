use crate::zerocopy::ZeroCopy;
use core::marker::PhantomData;
use cosmos_result::Code::InvalidArgument;
use crate::result::{err_code_raw, RawResult};

#[repr(C)]
pub struct Enum<T> {
    value: T,
    _phantom: PhantomData<T>,
}

unsafe impl<T: ZeroCopyEnum> ZeroCopy for Enum<T> {}

pub unsafe trait ZeroCopyEnum: Copy + Into<u8> + TryFrom<u8> {
    const MAX_VALUE: u8;
}

impl<T: ZeroCopyEnum> Enum<T> {
    fn get(&self) -> RawResult<T> {
        let value: u8 = self.value.into();
        if (value) > T::MAX_VALUE {
            err_code_raw(InvalidArgument)
        } else {
            Ok(self.value)
        }
    }

    fn set(&mut self, value: T) {
        self.value = value
    }
}

#[cfg(test)]
mod tests {
    use crate::r#enum::{Enum, ZeroCopyEnum};
    use core::marker::PhantomData;
    use core::mem::transmute;
    use num_enum::{IntoPrimitive, TryFromPrimitive};
    use crate::Code::InvalidArgument;
    use crate::result::err_code_raw;

    #[repr(u8)]
    #[derive(Clone, Copy, IntoPrimitive, TryFromPrimitive, Eq, PartialEq, Debug)]
    enum ABC {
        A,
        B,
        C,
    }

    unsafe impl ZeroCopyEnum for ABC {
        const MAX_VALUE: u8 = 2;
    }

    #[test]
    fn test_good() {
        let mut e = Enum::<ABC> {
            value: ABC::A,
            _phantom: PhantomData,
        };
        assert_eq!(e.get(), Ok(ABC::A));
        e.set(ABC::B);
        assert_eq!(e.get(), Ok(ABC::B));
    }

    #[test]
    fn test_bad() {
        let x: u8 = 3;
        let mut e: Enum<ABC> = unsafe { transmute(x) };
        assert_eq!(e.get(), err_code_raw(InvalidArgument));
        e.set(ABC::C);
        assert_eq!(e.get(), Ok(ABC::C));
    }
}
