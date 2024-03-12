use crate::{Code, Context, error};
use crate::handler::{Handler, HandlerWithResponse};
use crate::routing::{CallArgs, ServiceHandler};

impl<Req: prost::Name + Default> ServiceHandler for dyn Handler<Req> {
    fn invoke(&self, _method_id: u16, ctx: &mut Context, args: &mut CallArgs) -> crate::Result<()> {
        debug_assert_eq!(_method_id, 0);
        self.handle(ctx, &marshal_service_req(args)?)
    }
}

impl<Req: prost::Name + Default, Res: prost::Name> ServiceHandler for dyn HandlerWithResponse<Req, Res> {
    fn invoke(&self, _method_id: u16, ctx: &mut Context, args: &mut CallArgs) -> crate::Result<()> {
        debug_assert_eq!(_method_id, 0);
        marshal_service_res(args, self.handle(ctx, &marshal_service_req(args)?))
    }
}

pub fn marshal_service_req<T: prost::Message + Default>(args: &CallArgs) -> crate::Result<T> {
    // TODO figure out how to not decode if this is a local call
    T::decode(args.in1()).map_err(|e| error!(Code::InvalidArgument, "failed to decode request: {}", e))
}

pub fn marshal_service_res<T: prost::Message>(args: &mut CallArgs, res: crate::Result<T>) -> crate::Result<()> {
    // TODO figure out how to not encode if this is a local call
    res.map(|res| {
        args.set_out1(res.encode_to_vec());
    })
}
