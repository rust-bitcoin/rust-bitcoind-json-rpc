// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core v0.17.1 - generating.
//!
//! Types for methods found under the `== Generating ==` section of the API docs.

mod convert;

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `generatetoaddress`.
/// > generatetoaddress nblocks "address" ( maxtries )
/// >
/// > Mine blocks immediately to a specified address (before the RPC call returns)
/// >
/// > Arguments:
/// > 1. nblocks     (numeric, required) How many blocks are generated immediately.
/// > 2. address     (string, required) The address to send the newly generated bitcoin to.
/// > 3. maxtries    (numeric, optional, default=1000000) How many iterations to try.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GenerateToAddress(
    /// Hashes of blocks generated.
    pub Vec<String>,
);
