// SPDX-License-Identifier: CC0-1.0

//! Structs with standard types.
//!
//! These structs model the types returned by the JSON-RPC API and use stdlib types (or custom
//! types) and are specific to a specific to Bitcoin Core `v22.1`.

pub mod blockchain;
pub mod generating;
pub mod network;
pub mod wallet;

pub use blockchain::*;
pub use generating::*;
pub use network::*;
pub use wallet::*;
