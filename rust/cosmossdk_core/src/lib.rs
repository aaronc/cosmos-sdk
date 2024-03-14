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
mod result;
mod context;
pub mod module;
pub mod error;
pub mod sync;
pub mod bundle;
pub mod encoding;
pub mod account;
pub mod interface;
pub mod mem;
mod parallel;

pub use code::*;
pub use context::*;
pub use result::*;
pub use id::*;

// pub mod cosmos {
//     pub mod core {
//         pub mod v1alpha1 {
//             pub mod bundle {
//                 include!("cosmos/core/v1alpha1/bundle.rs");
//             }
//         }
//     }
// }

