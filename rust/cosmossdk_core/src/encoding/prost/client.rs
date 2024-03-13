use crate::routing::{Client, ClientConnection, ClientDescriptor, ClientDescriptorHelper, ClientFactory};

pub struct DynamicProstClient {
    conn: ClientConnection,
}

impl DynamicProstClient {
    pub fn invoke<T: prost::Name>() -> crate::Result<()> {
        todo!()
    }

    pub fn new_client<T: ProstClient>() -> T {
        todo!()
    }
}

impl Client for DynamicProstClient {
    fn describe(_helper: &mut dyn ClientDescriptorHelper) -> ClientDescriptor {
        ClientDescriptor::DynamicProtoClient
    }

    fn new(conn: ClientConnection) -> Self {
        DynamicProstClient {
            conn
        }
    }
}

// This is a marker trait for generated code that is a prost client.
pub trait ProstClient: Client {}