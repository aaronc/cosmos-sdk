mod key_codec;
mod value_codec;
mod errors;
mod buffer;
pub mod value;
pub mod kind;
mod field;
mod enum_type;
mod r#struct;
mod allocator;
mod binary;
mod visitor;
mod list;
mod codec;
mod schema;

// Re-export macros
#[cfg(feature = "macros")]
#[allow(unused_imports)]
#[macro_use]
extern crate cosmos_schema_macros;
#[cfg(feature = "macros")]
pub use cosmos_schema_macros::*;
