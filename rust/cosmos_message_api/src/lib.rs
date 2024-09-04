pub struct MessagePacket {
    header: MessageHeader,
}

pub struct MessageHeader {
    message_name: MessageName, // 128 bytes
    self_address: Address, // 64 bytes
    sender_address: Address, // 64 bytes
    context_token: [u8; 32], // 32 bytes
    state_token: StateToken, // 32 bytes
    message_name_hash: u64, // 8 bytes
    gas_limit: u64, // 8 bytes
    gas_consumed: u64, // 8 bytes
    in_pointer1: DataPointer, // 16 bytes
    in_pointer2: DataPointer, // 16 bytes
    out_pointer1: DataPointer, // 16 bytes
    out_pointer2: DataPointer, // 16 bytes
}

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

pub struct StateToken([u8; 32]);

impl StateToken {
    pub fn is_volatile(&self) -> bool {
        // value of first bit of first byte
        self.0[0] & 0x01 == 1
    }
}

pub struct MessageName {
    len: u8,
    data: [u8; 127],
}

pub struct DataPointer {
    native_pointer: u64,
    len: u32,
    offset_or_capacity: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Code {
    Ok,
    OutOfGas,
    FatalExecutionError,
    AccountNotFound,
    MessageHandlerNotFound,
    InvalidStateAccess,
    UnauthorizedCallerAccess,
    InvalidHandler,
    UnknownHandlerError,
    UnknownSystemError(u32),
    HandlerError(u32),
}

impl From<u32> for Code {
    fn from(value: u32) -> Self {
        match value {
            0 => Code::Ok,
            1 => Code::OutOfGas,
            2 => Code::FatalExecutionError,
            3 => Code::AccountNotFound,
            4 => Code::MessageHandlerNotFound,
            5 => Code::InvalidStateAccess,
            6 => Code::UnauthorizedCallerAccess,
            7 => Code::InvalidHandler,
            8 => Code::UnknownHandlerError,
            ..=255 => Code::UnknownSystemError(value),
            _ => Code::HandlerError(value),
        }
    }
}

impl Into<u32> for Code {
    fn into(self) -> u32 {
        match self {
            Code::Ok => 0,
            Code::OutOfGas => 1,
            Code::FatalExecutionError => 2,
            Code::AccountNotFound => 3,
            Code::MessageHandlerNotFound => 4,
            Code::InvalidStateAccess => 5,
            Code::UnauthorizedCallerAccess => 6,
            Code::InvalidHandler => 7,
            Code::UnknownHandlerError => 8,
            Code::UnknownSystemError(value) => value,
            Code::HandlerError(value) => value,
        }
    }
}

type Handler = unsafe fn(account_handler_id: u64, message_packet: *mut MessagePacket, packet_len: u32) -> u32;