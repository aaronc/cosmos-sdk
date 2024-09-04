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

