use crate::{Address, Context, ReadContext};
use crate::routing::ModuleServiceResolver;

pub trait AccountReadContext: ReadContext {
    fn account_id(&self) -> &Address;
}

pub trait AccountContext: Context + AccountReadContext {}

pub trait AccountHandler: ModuleServiceResolver + AccountCreateMessageHandler<Self::CreateMessage> {
    type CreateMessage;
}

pub trait AccountCreateMessageHandler<T> {
    fn create(&self, ctx: &dyn AccountContext, req: &T) -> crate::Result<()>;
}

pub trait AccountMessageHandler<T> {
    fn handle(&self, ctx: &dyn AccountContext, req: &T) -> crate::Result<()>;
}
