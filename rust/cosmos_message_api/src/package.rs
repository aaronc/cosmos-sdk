use crate::handler::{Handler, HandlerCode};
use crate::MessagePacket;

pub trait Package {
    fn num_handlers(&self) -> u64;
    fn handler(&self, handler_id: u64) -> Option<&dyn Handler>;
}