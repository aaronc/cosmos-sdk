use std::collections::HashMap;
use arrayvec::ArrayString;
use cosmos_core::Map;
use cosmos_message_api::{Address, MessageName, MessagePacket};

pub struct Hypervisor {
    vms: HashMap<String, Box<dyn VirtualMachine>>,
    state: HypervisorState,
}

pub struct HypervisorState {
    account_handlers: Map<Address, ArrayString<128>>,
    modules: Map<ArrayString<64>, Address>,
    module_configs: Map<ArrayString<64>, Vec<u8>>,
    module_message_accounts: Map<MessageName, Address>,
}

pub struct HandlerMetadata {
    messages: Vec<MessageMetadata>,
    state_config: Vec<u8>,
    extra: Vec<u8>
}

pub struct MessageMetadata {
    message_name: MessageName,
    volatility: Volatility,
    extra: Vec<u8>
}

pub enum Volatility {
    Pure,
    Readonly,
    Volatile,
}

impl Hypervisor {
    fn invoke(&self, message_packet: *mut MessagePacket, len: usize) -> u32 {
        todo!()
    }
}

pub type VMFactory = fn() -> Box<dyn VirtualMachine>;

pub trait VirtualMachine {
    fn name(&self) -> &str;
    fn invoke(&self, handler_id: &str, message_packet: *mut MessagePacket, len: usize) -> u32;
    fn handler_metadata(&self, handler_id: &str) -> HandlerMetadata;
}

pub trait StateHandler {
    fn create(&mut self, address: Address, config: Vec<u8>) -> u32;
    fn migrate(&mut self, address: Address, config: Vec<u8>) -> u32;
    fn destroy(&mut self, address: Address) -> u32;
}

pub struct NativeVM {}

pub trait NativeHandler {
    fn invoke(&self, message_packet: *mut MessagePacket, len: usize) -> u32;
}