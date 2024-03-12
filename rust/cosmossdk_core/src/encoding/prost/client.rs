use crate::routing::{Client, ClientConnection, ClientDescriptor, ClientDescriptorHelper, ClientFactory};

pub struct DynamicProstClient<'a> {
    conn: ClientConnection<'a>,
}

impl<'a> DynamicProstClient<'a> {
    pub fn invoke<T: prost::Name>() -> crate::Result<()> {
        todo!()
    }
}

impl<'a> Client<'a> for DynamicProstClient {
    fn describe(_helper: &mut dyn ClientDescriptorHelper) -> ClientDescriptor {
        ClientDescriptor::DynamicProtoClient
    }

    fn new(conn: ClientConnection) -> Self {
        DynamicProstClient {
            conn
        }
    }
}

trait ProstClient: Client {}