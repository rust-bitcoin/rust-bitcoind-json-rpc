// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core v22.1 - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

mod convert;

use serde::{Deserialize, Serialize};

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

/// Result of the JSON-RPC method `sendtoaddress`.
///
/// > sendtoaddress "address" amount ( "comment" "comment_to" subtractfeefromamount replaceable conf_target "estimate_mode" avoid_reuse fee_rate verbose )
/// >
/// > Send an amount to a given address.
/// > Requires wallet passphrase to be set with walletpassphrase call if wallet is encrypted.
/// >
/// > Arguments:
/// > 1. address                  (string, required) The bitcoin address to send to.
/// > 2. amount                   (numeric or string, required) The amount in BTC to send. eg 0.1
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SendToAddress {
    /// The transaction id.
    pub txid: String,
    /// The transaction fee reason.
    pub fee_reason: String,
}
