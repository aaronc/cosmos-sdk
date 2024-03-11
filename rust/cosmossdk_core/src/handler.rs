use crate::Context;
use crate::id::AgentId;
use crate::Result;

pub trait Handler<Request, Response = ()> {
    fn handle(&self, ctx: &mut Context, req: &Request) -> Result<Response>;
}

pub trait InternalHandler<Request, Response = ()> {
    fn handle(&self, ctx: &mut Context, caller_id: &AgentId, req: &Request) -> Result<Response>;
}

pub trait EventHook<Event> {
    fn on_event(&self, ctx: &mut Context, event: &Event) -> Result<()>;
}

pub trait PreHandler<Request> {
    fn pre_handle(&self, ctx: &mut Context, req: &Request) -> Result<()>;
}

pub trait PostHandler<Request, Response = ()> {
    fn post_handle(&self, ctx: &mut Context, req: &Request, res: &mut Response) -> Result<()>;
}