pub use cosmos_context::Context;
// pub use cosmos_schema::{StructCodec};

pub trait Message<'a> /*: StructCodec<'a>*/ {
    type Response;
    type Error;
}

pub trait MessageHandler<M: Message> {
    fn handle(&self, ctx: &mut Context, msg: &M) -> Result<M::Response, M::Error>;
}

pub trait QueryHandler<M: Message> {
    fn handle(&self, ctx: &Context, msg: &M) -> Result<M::Response, M::Error>;
}

// Re-export macros
#[cfg(feature = "core_macros")]
#[allow(unused_imports)]
#[macro_use]
extern crate cosmos_core_macros;
#[cfg(feature = "core_macros")]
pub use cosmos_core_macros::*;

#[cfg(feature = "schema_macros")]
#[allow(unused_imports)]
#[macro_use]
extern crate cosmos_schema_macros;
#[cfg(feature = "schema_macros")]
pub use cosmos_schema_macros::*;
