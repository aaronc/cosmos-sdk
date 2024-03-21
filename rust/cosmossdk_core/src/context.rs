use crate::{Address, AgentId};

/// Specifies the context for read-only calls.
pub trait ReadContext {
    type R: ClientRequest;

    /// Returns the unique identifier for the call that is scoped at the app level
    /// as either a user initiated transaction or an application lifecycle callback.
    fn id(&self) -> u64;

    /// Returns the account or module which has been invoked. This gives
    /// accounts the ability to know their address and modules the ability
    /// to know their module ID.
    fn self_id(&self) -> &AgentId;

    /// Creates a new request which can be used to make client calls from the context.
    fn new_request(&self) -> Self::R;

    /// Invokes a client call from the context.
    fn invoke(&self, req: &mut Self::R) -> crate::Result<()>;
}

/// Specifies a context for stateful calls - inherits all methods from
/// `ReadContext`.
pub trait Context: ReadContext {
    /// Returns the account or module which invoked the call.
    /// In the case of messages which include the signer in the
    /// message, this will always be equal to binary encoding of the
    /// signer address. In the case where there are multiple signers
    /// on a message, this will be an empty address and the method
    /// will need to get the signer from the message. A module ID
    /// will only be passed if a non-account module initiated the
    /// call using a message that does not include a signer field.
    fn caller_id(&self) -> &AgentId;
}

/// A request that can be used to make client calls from the context.
pub trait ClientRequest {
    type P: Param;

    /// Sets the fully qualified method name for the request.
    fn set_target_method(&mut self, route: &str) -> crate::Result<()>;

    /// Sets the target account address for the request when this call is a calling
    /// an account method. This method should not be called when calling a module method.
    fn set_target_account(&mut self, account: Address);

    fn in_params(&mut self) -> &mut [Self::P; 2];

    fn out_params(&self) -> &[Self::P; 2];
}

pub trait Param {
    fn bytes(&self) -> &[u8];
    fn set_bytes<'a>(&'a mut self, bytes: &'a [u8]);
}

pub trait ServerRequest {
    type Ctx: Context;
    type P: Param;

    fn context(&self) -> &Self::Ctx;

    fn in_params(&self) -> &[Self::P; 2];

    fn out_params(&mut self) -> &mut [Self::P; 2];
}