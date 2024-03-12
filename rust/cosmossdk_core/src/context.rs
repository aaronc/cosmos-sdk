use crate::AgentId;

#[repr(C)]
pub struct Context {
    pub(crate) id: u64,
    pub(crate) source: AgentId,
    pub(crate) target: AgentId,
    _padding: [u8; 508], // extra space for future use and makes context 1024 bytes
}

impl Context {
    /// Returns the unique identifier for the call that is scoped at the app level
    /// as either a user initiated transaction or an application lifecycle callback.
    fn id(&self) -> u64 {
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
