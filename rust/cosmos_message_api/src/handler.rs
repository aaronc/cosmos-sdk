use crate::{Code, MessagePacket};

pub trait Handler {
    fn handle(&self, message_packet: &mut MessagePacket, callbacks: &HostCallbacks) -> HandlerCode;
    // fn name() -> &'static str;
    // fn descriptor() -> &'static [u8];
}

#[non_exhaustive]
pub struct HostCallbacks {
    pub invoke: InvokeFn,
    pub log_fn: Option<LogFn>,
}

pub type InvokeFn = fn(&mut MessagePacket) -> Code;
pub type LogFn = fn(level: u8, msg: &str);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HandlerCode {
    Ok,
    HandlerError(u32),
}
