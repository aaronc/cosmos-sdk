use crate::Context;
use crate::id::AgentId;
use crate::Result;
use crate::routing::{CallArgs, ServiceHandler};

pub trait Handler<Request> {
    fn handle(&self, ctx: &mut Context, req: &Request) -> Result<()>;
}

pub trait HandlerWithResponse<Request, Response> {
    fn handle(&self, ctx: &mut Context, req: &Request) -> Result<Response>;
}

pub trait PreHandler<Request> {
    fn pre_handle(&self, ctx: &mut Context, req: &Request) -> Result<()>;
}

pub trait PostHandler<Request> {
    fn post_handle(&self, ctx: &mut Context, req: &Request) -> Result<()>;
}

pub trait PostHandlerWithResponse<Request, Response> {
    fn post_handle(&self, ctx: &mut Context, req: &Request, res: &mut Response) -> Result<()>;
}

pub trait EventHook<Event> {
    fn on_event(&self, ctx: &mut Context, event: &Event) -> Result<()>;
}

