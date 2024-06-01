// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core v22.1 - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.


mod convert;

use serde::{Deserialize, Serialize};

/// Result of the JSON-RPC method `createwallet`.
///
/// > createwallet "wallet_name" ( disable_private_keys blank "passphrase" avoid_reuse descriptors load_on_startup external_signer )
/// >
/// > Creates and loads a new wallet.
/// >
/// > Arguments:
/// > 1. wallet_name             (string, required) The name for the new wallet. If this is a path, the wallet will be created at the path location.
/// > 2. disable_private_keys    (boolean, optional, default=false) Disable the possibility of private keys (only watchonlys are possible in this mode).
/// > 3. blank                   (boolean, optional, default=false) Create a blank wallet. A blank wallet has no keys or HD seed. One can be set using sethdseed.
/// > 4. passphrase              (string, optional) Encrypt the wallet with this passphrase.
/// > 5. avoid_reuse             (boolean, optional, default=false) Keep track of coin reuse, and treat dirty and clean coins differently with privacy considerations in mind.
/// > 6. descriptors             (boolean, optional, default=true) Create a native descriptor wallet. The wallet will use descriptors internally to handle address creation. Setting to "false" will create a legacy wallet; This is only possible with the -deprecatedrpc=create_bdb setting because, the legacy wallet type is being deprecated and support for creating and opening legacy wallets will be removed in the future.
/// > 7. load_on_startup         (boolean, optional) Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
/// > 8. external_signer         (boolean, optional, default=false) Use an external signer such as a hardware wallet. Requires -signer to be configured. Wallet creation will fail if keys cannot be fetched. Requires disable_private_keys and descriptors set to true.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CreateWallet {
    /// The wallet name if created successfully.
    ///
    /// If the wallet was created using a full path, the wallet_name will be the full path.
    pub name: String,
    /// Warning messages, if any, related to creating and loading the wallet.
    pub warning: String,
}

/// Result of the JSON-RPC method `loadwallet`.
///
/// > loadwallet "filename" ( load_on_startup )
/// >
/// > Loads a wallet from a wallet file or directory.
/// > Note that all wallet command-line options used when starting bitcoind will be
/// > applied to the new wallet.
/// >
/// > Arguments:
/// > 1. filename           (string, required) The wallet directory or .dat file.
/// > 2. load_on_startup    (boolean, optional) Save wallet name to persistent settings and load on startup. True to add wallet to startup list, false to remove, null to leave unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LoadWallet {
    /// The wallet name if loaded successfully.
    pub name: String,
    /// Warning messages, if any, related to loading the wallet.
    pub warning: String,
}

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

/// Result of the JSON-RPC method `getbalance`.
///
/// > getbalance ( "dummy" minconf include_watchonly avoid_reuse )
/// >
/// > Returns the total available balance.
/// > The available balance is what the wallet considers currently spendable, and is
/// > thus affected by options which limit spendability such as -spendzeroconfchange.
/// >
/// > Arguments:
/// > 1. dummy                (string, optional) Remains for backward compatibility. Must be excluded or set to "*".
/// > 2. minconf              (numeric, optional, default=0) Only include transactions confirmed at least this many times.
/// > 3. include_watchonly    (boolean, optional, default=true for watch-only wallets, otherwise false) Also include balance in watch-only addresses (see 'importaddress')
/// > 4. avoid_reuse          (boolean, optional, default=true) (only available if avoid_reuse wallet flag is set) Do not include balance in dirty outputs; addresses are considered dirty if they have previously been used in a transaction.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBalance(pub f64);

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
