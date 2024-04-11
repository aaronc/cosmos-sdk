extern crate alloc;

pub use address::*;
pub use client_request::*;
pub use param::*;

use crate::AgentId;
use cosmos_result::Result;

mod param;
mod client_request;
mod address;
mod context;

/// Specifies the context for read-only calls.
pub trait ReadContext<'a> {
    type R: ClientRequest<'a>;

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
    fn invoke(&self, req: &mut Self::R) -> Result<()>;
}

/// Specifies a context for stateful calls - inherits all methods from
/// `ReadContext`.
pub trait Context<'a>: ReadContext<'a> {
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
