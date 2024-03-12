use crate::Context;
use crate::handler::Handler;
use crate::routing::{CallArgs, ServiceHandler};

impl<Req: prost::Message + prost::Name, Res> ServiceHandler for dyn Handler<Req, Res> {
    fn invoke(&self, _method_id: u16, ctx: &mut Context, args: &mut CallArgs) -> crate::Result<()> {
        debug_assert_eq!(_method_id, 0);
        marshal_service_res(args, self.handle(ctx, &marshal_service_req(args)?))
    }
}

pub fn marshal_service_req<T: prost::Message>(args: &CallArgs) -> crate::Result<T> {
    T::decode(args.in1()).map_err(|e| e.into())
}

pub fn marshal_service_res<T: prost::Message>(args: &mut CallArgs, res: crate::Result<T>) -> crate::Result<()> {
    res.map(|res| {
        args.set_out1(res.encode_to_vec());
    })
}
