use prost::Message;
use crate::{AgentId, Code, Context, err, error, ModuleId, Param, ReadContext, ServerRequest, ServerRequestWrapper};
use crate::module::{MessageHandler, MessageHandlerWithResponse, ModuleContext, ModuleReadContext};
use crate::routing::{CallArgs, ContextData, ContextImpl, ModuleContextImpl, ServerRequestImpl, ServerRequestWrapperImpl, ServiceHandler};

impl<Req: prost::Name + Default> ServiceHandler<ServerRequestWrapperImpl> for dyn MessageHandler<Req, ModuleContextImpl<'_>>
{
    fn invoke(&self, _method_id: u32, sr: &mut ServerRequestImpl) -> crate::Result<()> {
        debug_assert_eq!(_method_id, 0);
        let args = sr.in_params()[0].bytes();
        let req = Req::decode(args).map_err(|e| error!(Code::InvalidArgument, "failed to decode request: {}", e))?;
        let context = ModuleContextImpl::new(sr.context())?;
        let res = self.handle(&context, &req)?;
        sr.out_params().set_bytes(res.encode_to_vec());
        Ok(())
    }
}

// impl<Req: prost::Name + Default, Res: prost::Name> ServiceHandler for dyn MessageHandlerWithResponse<Req, Res, ModuleContextData<'_>> {
//     fn invoke(&self, _method_id: u32, ctx: &ContextData, args: &mut CallArgs) -> crate::Result<()> {
//         debug_assert_eq!(_method_id, 0);
//         let ctx = ModuleContextData::new(ctx)?;
//         marshal_service_res(args, self.handle(&ctx, &marshal_service_req(args)?))
//     }
// }

// pub fn marshal_service_req<T: prost::Message + Default>(args: &CallArgs) -> crate::Result<T> {
//     // TODO figure out how to not decode if this is a local call
//     T::decode(args.in1()).map_err(|e| error!(Code::InvalidArgument, "failed to decode request: {}", e))
// }
//
// pub fn marshal_service_res<T: prost::Message>(args: &mut CallArgs, res: crate::Result<T>) -> crate::Result<()> {
//     // TODO figure out how to not encode if this is a local call
//     res.map(|res| {
//         args.set_out1(res.encode_to_vec());
//     })
// }

// struct ModuleContextData<'a> {
//     context: &'a ContextData,
//     module_id: &'a ModuleId,
// }
//
// impl <'a> ModuleContextData<'a> {
//     pub fn new(context: &'a ContextData) -> crate::Result<Self> {
//         let AgentId::Module(module_id) = &context.target else {
//             return err!(Code::Internal, "ModuleContextData::new: target is not a module")
//         };
//         Ok(ModuleContextData {
//             context,
//             module_id,
//         })
//     }
// }

// impl <'a> ReadContext for ModuleContextData<'a> {
//     type R = ();
//
//     fn id(&self) -> u64 {
//         self.context.id
//     }
//
//     fn self_id(&self) -> &AgentId {
//         &self.context.target
//     }
//
//     fn new_request(&self) -> Self::R {
//         todo!()
//     }
//
//     fn invoke(&self, req: &mut Self::R) -> crate::Result<()> {
//         todo!()
//     }
// }
//
// impl <'a> Context for ModuleContextData<'a> {
//     fn caller_id(&self) -> &AgentId {
//         &self.context.source
//     }
// }
//
// impl <'a> ModuleReadContext for ModuleContextData<'a> {
//     fn module_id(&self) -> &ModuleId {
//         &self.module_id
//     }
// }
// impl <'a> ModuleContext for ModuleContextData<'a> {}
