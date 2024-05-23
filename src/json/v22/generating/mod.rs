// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the wallet section of the API docs.
//!
//! The JSON-RPC API for Bitcoin Core v22.1:
//!
//!  == Generating ==
//!
//! - [ ] generateblock "output" ["rawtx/txid",...]
//! - [x] generatetoaddress nblocks "address" ( maxtries )
//! - [ ] generatetodescriptor num_blocks "descriptor" ( maxtries )

mod convert; // Currently unused.

pub use crate::json::v17::GenerateToAddress;
