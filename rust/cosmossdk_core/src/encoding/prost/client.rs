use crate::routing::{Client, ClientConnection, ClientDescriptor, ClientDescriptorHelper, ClientFactory};

pub struct DynamicProstClient;

impl Client for DynamicProstClient {
    fn describe(_helper: &mut dyn ClientDescriptorHelper) -> ClientDescriptor {
        ClientDescriptor::DynamicProtoClient
    }
}

// This is a marker trait for generated code that is a prost client.
pub trait ProstClient: Client {}