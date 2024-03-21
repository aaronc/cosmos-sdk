use crate::module::ModuleContext;
use crate::Result;

pub trait MessageHandler<Request, Ctx: ModuleContext> {
    fn handle(&self, ctx: &Ctx, req: &Request) -> Result<()>;
}

pub trait MessageHandlerWithResponse<Request, Response> {
    fn handle(&self, ctx: &dyn ModuleContext, req: &Request) -> Result<Response>;
}

pub trait MessagePreHandler<Request> {
    fn pre_handle(&self, ctx: &dyn ModuleContext, req: &Request) -> Result<()>;
}

pub trait MessagePostHandler<Request> {
    fn post_handle(&self, ctx: &dyn ModuleContext, req: &Request) -> Result<()>;
}

pub trait PostHandlerWithResponse<Request, Response> {
    fn post_handle(&self, ctx: &dyn ModuleContext, req: &Request, res: &mut Response) -> Result<()>;
}

pub trait EventHook<Event> {
    fn on_event(&self, ctx: &dyn ModuleContext, event: &Event) -> Result<()>;
}

