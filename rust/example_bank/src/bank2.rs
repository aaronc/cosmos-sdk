// struct Context {
//
// }
//
// struct BankContext {
// }
//
// pub trait Coin {
//     fn send(&self, ctx: &Context, bank_context: &BankContext) -> cosmossdk_core::Result<()>;
// }

pub struct EventSend {
    from: String,
    to: String,
    denom: String,
    amount: String,
}

pub struct Context;

pub struct Bank {}
pub trait EventHook<Event> {
    fn on_event(&self, ctx: &Context, event: &Event) -> Result<(), ()>;
}

impl EventHook<EventSend> for Bank {
    fn on_event(&self, ctx: &Context, event: &EventSend) -> Result<(), ()> {
        todo!()
    }
}