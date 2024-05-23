// SPDX-License-Identifier: CC0-1.0

//! Support for connecting to Bitcoin Core via JSON-RPC.

/// Re-export the `rust-bitcoin` crate.
pub extern crate bitcoin;

// TODO: Consider updating https://en.bitcoin.it/wiki/API_reference_%28JSON-RPC%29 when this is complete.

#[cfg(feature = "client-sync")]
#[macro_use]
pub mod client_sync;
pub mod json;
pub mod model;
