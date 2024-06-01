// SPDX-License-Identifier: CC0-1.0

//! Structs with standard types.
//!
//! These structs model the types returned by the JSON-RPC API and use stdlib types (or custom
//! types) and are specific to a specific to Bitcoin Core `v26.1`.

pub mod blockchain;
pub mod network;
pub mod wallet;

#[rustfmt::skip]
#[doc(inline)]
pub use self::{
    blockchain::*,
    network::*,
    wallet::*,
};
