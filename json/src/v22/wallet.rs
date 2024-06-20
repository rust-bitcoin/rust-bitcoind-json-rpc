// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core v22 - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

use serde::{Deserialize, Serialize};

use crate::model;

/// Result of the JSON-RPC method `unloadwallet`.
///
/// > unloadwallet ( "wallet_name" load_on_startup )
/// >
/// > Unloads the wallet referenced by the request endpoint, otherwise unloads the wallet specified in the argument.
/// > Specifying the wallet name on a wallet endpoint is invalid.
/// >
/// > Arguments:
/// > 1. wallet_name        (string, optional, default=the wallet name from the RPC endpoint) The name of the wallet to unload. If provided both here and in the RPC endpoint, the two must be identical.
/// > 2. load_on_startup    (boolean, optional) Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UnloadWallet {
    /// Warning messages, if any, related to unloading the wallet.
    pub warning: String,
}

impl UnloadWallet {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::UnloadWallet {
        model::UnloadWallet { warnings: vec![self.warning] }
    }
}
