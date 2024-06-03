// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core v0.19.1 - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

mod convert;

use serde::{Deserialize, Serialize};

/// Result of the JSON-RPC method `getbalances`.
///
/// > getbalances
/// >
/// > Returns an object with all balances in BTC.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBalances {
    /// Balances from outputs that the wallet can sign.
    pub mine: GetBalancesMine,
    #[serde(rename = "watchonly")]
    pub watch_only: Option<GetBalancesWatchOnly>,
}

/// Balances from outputs that the wallet can sign.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBalancesMine {
    /// Trusted balance (outputs created by the wallet or confirmed outputs).
    pub trusted: f64,
    /// Untrusted pending balance (outputs created by others that are in the mempool).
    pub untrusted_pending: f64,
    /// Balance from immature coinbase outputs.
    pub immature: f64,
    /// Balance from coins sent to addresses that were previously spent from (potentially privacy violating).
    ///
    /// Only present if `avoid_reuse` is set.
    pub used: Option<f64>,
}

/// Hash and height of the block this information was generated on.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBalancesWatchOnly {
    /// Trusted balance (outputs created by the wallet or confirmed outputs).
    pub trusted: f64,
    /// Untrusted pending balance (outputs created by others that are in the mempool).
    pub untrusted_pending: f64,
    /// Balance from immature coinbase outputs.
    pub immature: f64,
}
