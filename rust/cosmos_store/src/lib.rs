use cosmos_context::Context;
use cosmos_message_api::MessagePacket;
// pub trait State {
// }

pub trait KVStore {
    fn get(&self, ctx: &mut Context) -> u32;
    fn set(&self, ctx: &mut Context) -> u32;
    fn delete(&self, ctx: &mut Context) -> u32;
}