use crate::{ClientRequest, Context, Param};
use crate::routing::{Client, ClientDescriptor, ClientDescriptorHelper};

pub struct DynamicProstClient;

impl DynamicProstClient {
    pub fn invoke<Ctx: Context, T: prost::Name>(ctx: &Ctx, t: &T) -> crate::Result<()> {
        let mut req = ctx.new_request();
        req.set_target_method(&T::full_name());
        req.in_params()[0].set_bytes(t.encode_to_vec().as_slice());
        ctx.invoke(&mut req)
    }
}

impl Client for DynamicProstClient {
    fn describe(_helper: &mut dyn ClientDescriptorHelper) -> ClientDescriptor {
        ClientDescriptor::DynamicProtoClient
    }
}