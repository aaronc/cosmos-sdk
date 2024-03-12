use crate::routing::{Client, ClientConnection, ClientDescriptor, ClientDescriptorHelper, ClientFactory};

pub struct DynamicProstClient<'a> {
    conn: ClientConnection<'a>,
}

impl<'a> DynamicProstClient<'a> {
    pub fn invoke<T: prost::Name>() -> crate::Result<()> {
        todo!()
    }

    pub fn new_client<T: ProstClient<'a>>() -> T {
        todo!()
    }
}

impl<'a> Client<'a> for DynamicProstClient<'a> {
    fn describe(_helper: &mut dyn ClientDescriptorHelper) -> ClientDescriptor {
        ClientDescriptor::DynamicProtoClient
    }

    fn new(conn: ClientConnection<'a>) -> Self {
        DynamicProstClient {
            conn
        }
    }
}

// This is a marker trait for generated code that is a prost client.
pub trait ProstClient<'a>: Client<'a> {}