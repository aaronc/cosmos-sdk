use cosmos_message_api::account_handler::{AccountHandler, HostCallbacks, InvokeFn, LogFn};
use cosmos_message_api::MessagePacket;

pub trait VM {
    fn handler(handler_id: &str) -> &dyn AccountHandler;
}

pub trait VMFactory {
    fn create_vm(&self, callbacks: HostCallbacks) -> Box<dyn VM>;
}

pub trait StateHandler {

}

pub trait AuthorizationHandler {

}