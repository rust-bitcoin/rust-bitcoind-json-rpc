// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the `== Wallet ==` section of the API docs.
//!
//! These structs model the types returned by the JSON-RPC API but have concrete types
//! and are not specific to a specific version of Bitcoin Core.

use bitcoin::address::{Address, NetworkUnchecked};
use bitcoin::{Amount, SignedAmount, Transaction, Txid};
use serde::{Deserialize, Serialize};

/// Models the result of JSON-RPC method  `createwallet`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CreateWallet {
    /// The wallet name if created successfully.
    ///
    /// If the wallet was created using a full path, the wallet_name will be the full path.
    pub name: String,
    /// Warning messages, if any, related to creating and loading the wallet.
    pub warnings: Vec<String>,
}

/// Models the result of JSON-RPC method `loadwallet`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LoadWallet {
    /// The wallet name if loaded successfully.
    pub name: String,
    /// Warning messages, if any, related to loading the wallet.
    pub warnings: Vec<String>,
}

/// Models the result of JSON-RPC method `unloadwallet`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UnloadWallet {
    /// Warning messages, if any, related to unloading the wallet.
    pub warnings: Vec<String>,
}

/// Models the result of JSON-RPC method `getbalance`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBalance(pub Amount);

/// Models the result of JSON-RPC method `getbalances`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBalances {
    /// Balances from outputs that the wallet can sign.
    pub mine: GetBalancesMine,
    pub watch_only: Option<GetBalancesWatchOnly>,
}

/// Balances from outputs that the wallet can sign.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBalancesMine {
    /// Trusted balance (outputs created by the wallet or confirmed outputs).
    pub trusted: Amount,
    /// Untrusted pending balance (outputs created by others that are in the mempool).
    pub untrusted_pending: Amount,
    /// Balance from immature coinbase outputs.
    pub immature: Amount,
    /// Balance from coins sent to addresses that were previously spent from (potentially privacy violating).
    ///
    /// Only present if `avoid_reuse` is set.
    pub used: Option<Amount>,
}

/// Hash and height of the block this information was generated on.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBalancesWatchOnly {
    /// Trusted balance (outputs created by the wallet or confirmed outputs).
    pub trusted: Amount,
    /// Untrusted pending balance (outputs created by others that are in the mempool).
    pub untrusted_pending: Amount,
    /// Balance from immature coinbase outputs.
    pub immature: Amount,
}

/// Models the result of JSON-RPC method `getnewaddress`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetNewAddress(pub Address<NetworkUnchecked>);

/// Models the result of JSON-RPC method `sendtoaddress`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SendToAddress {
    /// The transaction id.
    pub txid: Txid,
}

/// Models the result of JSON-RPC method `gettransaction`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetTransaction {
    #[serde(default, with = "bitcoin::amount::serde::as_btc")]
    pub amount: SignedAmount,
    #[serde(default, with = "bitcoin::amount::serde::as_btc::opt")]
    pub fee: Option<SignedAmount>,
    pub confirmations: u32,
    pub txid: Txid,
    pub time: u64,
    pub time_received: u64,
    pub bip125_replaceable: String,
    pub details: Vec<GetTransactionDetail>,
    pub tx: Transaction,
}

/// Part of the `GetTransaction`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetTransactionDetail {
    pub address: Address<NetworkUnchecked>,
    pub category: GetTransactionDetailCategory,
    #[serde(default, with = "bitcoin::amount::serde::as_btc")]
    pub amount: SignedAmount,
    pub label: Option<String>,
    pub vout: u32,
    #[serde(default, with = "bitcoin::amount::serde::as_btc::opt")]
    pub fee: Option<SignedAmount>,
    pub abandoned: Option<bool>,
}

/// Enum to represent the category of a transaction.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum GetTransactionDetailCategory {
    Send,
    Receive,
    Generate,
    Immature,
    Orphan,
}
