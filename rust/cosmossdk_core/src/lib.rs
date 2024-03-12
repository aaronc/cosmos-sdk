#![cfg_attr(feature = "no_std", no_std)]

// #![no_implicit_prelude]

#[cfg(feature="alloc")]
extern crate alloc;
extern crate core;

#[cfg(target_arch = "wasm32")]
mod wasm;


#[cfg(any(test, feature = "test-util"))]
pub mod testing;
pub mod store;
pub mod routing;

mod code;
mod id;
mod handler;
mod result;
mod context;
pub mod module;
pub mod error;
pub mod sync;
pub mod bundle;
pub mod encoding;
mod account;
mod interface;

pub use code::Code;
pub use context::{Context};
pub use result::{Result, ok};
pub use id::{AgentId};

// pub mod cosmos {
//     pub mod core {
//         pub mod v1alpha1 {
//             pub mod bundle {
//                 include!("cosmos/core/v1alpha1/bundle.rs");
//             }
//         }
//     }
// }

