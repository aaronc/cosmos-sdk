use crate::AgentId;
use crate::raw::BytesPtr;

#[repr(C)]
pub struct Context {
    id: u64,
    route_info: RouteInfo,
    source: AgentId,
    target: AgentId,
    in1: BytesPtr,
    in2: BytesPtr,
    out1: BytesPtr,
    out2: BytesPtr,
}

impl Context {
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Returns the account or module which invoked the call.
    /// In the case of messages which include the signer in the
    /// message, this will always be equal to binary encoding of the
    /// signer address. In the case where there are multiple signers
    /// on a message, this will be an empty address and the method
    /// will need to get the signer from the message. A module ID
    /// will only be passed if a non-account module initiated the
    /// call using a message that does not include a signer field.
    pub fn caller_id(&self) -> &AgentId {
        &self.source
    }

    // Returns the account or module which has been invoked. This gives
    // accounts the ability to know their address and modules the ability
    // to know their module ID.
    pub fn self_id(&self) -> &AgentId {
        &self.target
    }
}

#[repr(C)]
pub(crate) struct RouteInfo {
    module_index: u32,
    service_index: u16,
    method_index: u16,
}