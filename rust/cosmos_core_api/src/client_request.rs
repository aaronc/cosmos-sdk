use crate::address::Address;
use crate::param::Param;
use cosmos_result::{Result};

/// A request that can be used to make client calls from the context.
pub trait ClientRequest<'a> {
    type P: Param<'a>;

    /// Sets the fully qualified method name for the request.
    fn set_target_method(&mut self, route: &str) -> Result<()>;

    /// Sets the target account address for the request when this call is a calling
    /// an account method. This method should not be called when calling a module method.
    fn set_target_account(&mut self, account: Address);

    fn in_params(&mut self) -> &mut [Self::P; 2];

    fn out_params(&self) -> &[Self::P; 2];
}
