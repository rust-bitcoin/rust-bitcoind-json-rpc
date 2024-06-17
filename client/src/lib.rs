// SPDX-License-Identifier: CC0-1.0

//! Support for connecting to Bitcoin Core via JSON-RPC.

/// Re-export the `rust-bitcoin` crate.
pub extern crate bitcoin;

/// Re-export the `rust-bitcoin-json-rpc-types` crate.
pub extern crate json;

#[cfg(feature = "client-sync")]
#[macro_use]
pub mod client_sync;
