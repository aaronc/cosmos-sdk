mod address;
mod package;
pub mod account_handler;
mod wasm;

pub use address::Address;

pub struct MessagePacket {
    data: *mut u8,
    len: usize,
}

pub struct MessageHeader {
    pub message_name: MessageName, // 128 bytes
    pub target_address: Address, // 64 bytes
    pub sender_address: Address, // 64 bytes
    pub context_token: [u8; 32], // 32 bytes
    pub state_token: StateToken, // 32 bytes
    pub message_name_hash: u64, // 8 bytes
    pub gas_limit: u64, // 8 bytes
    pub gas_consumed: u64, // 8 bytes
    pub in_pointer1: DataPointer, // 16 bytes
    pub in_pointer2: DataPointer, // 16 bytes
    pub out_pointer1: DataPointer, // 16 bytes
    pub out_pointer2: DataPointer, // 16 bytes
}

impl MessagePacket {
    pub unsafe fn new(data: *mut u8, len: usize) -> Self {
        Self { data, len }
    }

    pub unsafe fn header(&self) -> &MessageHeader {
        &*(self.data as *const MessageHeader)
    }

    pub unsafe fn header_mut(&self) -> &mut MessageHeader {
        &mut *(self.data as *mut MessageHeader)
    }

    pub unsafe fn in_data1(&self) -> &[u8] {
        self.header().in_pointer1.data(self.data, self.len)
    }

    pub unsafe fn in_data2(&self) -> &[u8] {
        self.header().in_pointer2.data(self.data, self.len)
    }
}

pub const MESSAGE_HEADER_SIZE: usize = 512;

#[derive(Clone, Copy, PartialEq, Eq)]
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
    pub native_pointer: u64,
    pub len: u32,
    pub offset_or_capacity: u32,
}

impl DataPointer {
    pub unsafe fn data(&self, message_packet: *const u8, packet_len: usize) -> &[u8] {
        if self.native_pointer == 0 {
            if self.offset_or_capacity < MESSAGE_HEADER_SIZE as u32 {
                return &[];
            }
            if (self.offset_or_capacity + self.len) as usize > packet_len {
                return &[];
            }
            unsafe {
                return core::slice::from_raw_parts(message_packet.offset(self.offset_or_capacity as isize), self.len as usize);
            }
        }
        unsafe {
            core::slice::from_raw_parts(self.native_pointer as *const u8, self.len as usize)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SystemErrorCode {
    OutOfGas,
    FatalExecutionError,
    AccountNotFound,
    MessageHandlerNotFound,
    InvalidStateAccess,
    UnauthorizedCallerAccess,
    InvalidHandler,
    UnknownHandlerError,
    Unknown(u32),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Code {
    Ok,
    SystemError(SystemErrorCode),
    HandlerError(u32),
}

// impl From<u32> for Code {
//     fn from(value: u32) -> Self {
//         match value {
//             0 => Code::Ok,
//             1 => Code::OutOfGas,
//             2 => Code::FatalExecutionError,
//             3 => Code::AccountNotFound,
//             4 => Code::MessageHandlerNotFound,
//             5 => Code::InvalidStateAccess,
//             6 => Code::UnauthorizedCallerAccess,
//             7 => Code::InvalidHandler,
//             8 => Code::UnknownHandlerError,
//             ..=255 => Code::UnknownSystemError(value),
//             _ => Code::HandlerError(value),
//         }
//     }
// }
//
// impl Into<u32> for Code {
//     fn into(self) -> u32 {
//         match self {
//             Code::Ok => 0,
//             Code::OutOfGas => 1,
//             Code::FatalExecutionError => 2,
//             Code::AccountNotFound => 3,
//             Code::MessageHandlerNotFound => 4,
//             Code::InvalidStateAccess => 5,
//             Code::UnauthorizedCallerAccess => 6,
//             Code::InvalidHandler => 7,
//             Code::UnknownHandlerError => 8,
//             Code::UnknownSystemError(value) => value,
//             Code::HandlerError(value) => value,
//         }
//     }
// }

// pub type Handler = unsafe fn(account_handler_id: u64, message_packet: *mut u8, packet_len: u32) -> u32;
//
// pub type InvokeFn = unsafe fn(message_packet: *mut u8, packet_len: u32) -> u32;

