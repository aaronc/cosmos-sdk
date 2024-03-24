extern crate core;
extern crate num_enum;
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Clone, Copy, Eq, PartialEq, Debug, IntoPrimitive, FromPrimitive)]
#[repr(u8)]
pub enum Code {
    Ok = 0,
    Cancelled = 1,
    #[num_enum(default)]
    Unknown = 2,
    InvalidArgument = 3,
    DeadlineExceeded = 4,
    NotFound = 5,
    AlreadyExists = 6,
    PermissionDenied = 7,
    ResourceExhausted = 8,
    FailedPrecondition = 9,
    Aborted = 10,
    OutOfRange = 11,
    Unimplemented = 12,
    Internal = 13,
    Unavailable = 14,
    DataLoss = 15,
    Unauthenticated = 16,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", super::Code::Ok), "Ok");
    }
}
