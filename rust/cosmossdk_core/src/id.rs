extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentId {
    Module(String),
    Account(Vec<u8>)
}