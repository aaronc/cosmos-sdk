use crate::Context;
use crate::routing::ModuleServiceResolver;

pub trait AccountHandler: ModuleServiceResolver + CreateAccountHandler<Self::CreateMessage> {
    type CreateMessage;
}

pub trait CreateAccountHandler<T> {
    fn create(&self, ctx: &mut Context, req: &T) -> crate::Result<()>;
}
