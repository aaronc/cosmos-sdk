extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

pub enum AgentId {
    Module(String),
    Account(Vec<u8>)
}