// SPDX-License-Identifier: CC0-1.0

//! Types returned by the JSON-RPC API of Bitcoin Core.
//!
//! Note, the types here are specific the the `bitcoind` version and we currently only support the
//! latest minor version number for each major release i.e., `v22` means `bitcoind v22.1`.

pub mod v17;
pub mod v18;
pub mod v22;
