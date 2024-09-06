// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core v0.19.1 - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

use bitcoin::amount::ParseAmountError;
use bitcoin::Amount;
use serde::{Deserialize, Serialize};

use crate::model;

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

impl GetBalances {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBalances, ParseAmountError> {
        let mine = self.mine.into_model()?;
        let watch_only = self.watch_only.map(|watch_only| watch_only.into_model()).transpose()?;

        Ok(model::GetBalances { mine, watch_only })
    }
}

impl GetBalancesMine {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBalancesMine, ParseAmountError> {
        let trusted = Amount::from_btc(self.trusted)?;
        let untrusted_pending = Amount::from_btc(self.untrusted_pending)?;
        let immature = Amount::from_btc(self.immature)?;
        let used = self.used.map(Amount::from_btc).transpose()?;

        Ok(model::GetBalancesMine { trusted, untrusted_pending, immature, used })
    }
}

impl GetBalancesWatchOnly {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBalancesWatchOnly, ParseAmountError> {
        let trusted = Amount::from_btc(self.trusted)?;
        let untrusted_pending = Amount::from_btc(self.untrusted_pending)?;
        let immature = Amount::from_btc(self.immature)?;

        Ok(model::GetBalancesWatchOnly { trusted, untrusted_pending, immature })
    }
}
