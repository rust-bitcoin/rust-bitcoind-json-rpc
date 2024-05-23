// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the wallet section of the API docs.
//!
//! The JSON-RPC API for Bitcoin Core v0.17.1:
//!
//!  == Wallet ==
//!
//! - [ ] abandontransaction "txid"
//! - [ ] abortrescan
//! - [ ] addmultisigaddress nrequired ["key",...] ( "label" "address_type" )
//! - [ ] backupwallet "destination"
//! - [ ] bumpfee "txid" ( options )
//! - [x] createwallet "wallet_name" ( disable_private_keys )
//! - [ ] dumpprivkey "address"
//! - [ ] dumpwallet "filename"
//! - [ ] encryptwallet "passphrase"
//! - [ ] getaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)
//! - [ ] getaccountaddress (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)
//! - [ ] getaddressbyaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)
//! - [ ] getaddressesbylabel "label"
//! - [ ] getaddressinfo "address"
//! - [ ] getbalance ( "(dummy)" minconf include_watchonly )
//! - [ ] getnewaddress ( "label" "address_type" )
//! - [ ] getrawchangeaddress ( "address_type" )
//! - [ ] getreceivedbyaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)
//! - [ ] getreceivedbyaddress "address" ( minconf )
//! - [ ] gettransaction "txid" ( include_watchonly )
//! - [ ] getunconfirmedbalance
//! - [ ] getwalletinfo
//! - [ ] importaddress "address" ( "label" rescan p2sh )
//! - [ ] importmulti "requests" ( "options" )
//! - [ ] importprivkey "privkey" ( "label" ) ( rescan )
//! - [ ] importprunedfunds
//! - [ ] importpubkey "pubkey" ( "label" rescan )
//! - [ ] importwallet "filename"
//! - [ ] keypoolrefill ( newsize )
//! - [ ] listaccounts (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)
//! - [ ] listaddressgroupings
//! - [ ] listlabels ( "purpose" )
//! - [ ] listlockunspent
//! - [ ] listreceivedbyaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)
//! - [ ] listreceivedbyaddress ( minconf include_empty include_watchonly address_filter )
//! - [ ] listsinceblock ( "blockhash" target_confirmations include_watchonly include_removed )
//! - [ ] listtransactions (label count skip include_watchonly)
//! - [ ] listunspent ( minconf maxconf  ["addresses",...] [include_unsafe] [query_options])
//! - [ ] listwallets
//! - [ ] loadwallet "filename"
//! - [ ] lockunspent unlock ([{"txid":"txid","vout":n},...])
//! - [ ] move (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)
//! - [ ] removeprunedfunds "txid"
//! - [ ] rescanblockchain ("start_height") ("stop_height")
//! - [ ] sendfrom (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)
//! - [ ] sendmany "" {"address":amount,...} ( minconf "comment" ["address",...] replaceable conf_target "estimate_mode")
//! - [ ] sendtoaddress "address" amount ( "comment" "comment_to" subtractfeefromamount replaceable conf_target "estimate_mode")
//! - [ ] setaccount (Deprecated, will be removed in V0.18. To use this command, start bitcoind with -deprecatedrpc=accounts)
//! - [ ] sethdseed ( "newkeypool" "seed" )
//! - [ ] settxfee amount
//! - [ ] signmessage "address" "message"
//! - [ ] signrawtransactionwithwallet "hexstring" ( [{"txid":"id","vout":n,"scriptPubKey":"hex","redeemScript":"hex"},...] sighashtype )
//! - [ ] unloadwallet ( "wallet_name" )
//! - [ ] walletcreatefundedpsbt [{"txid":"id","vout":n},...] [{"address":amount},{"data":"hex"},...] ( locktime ) ( replaceable ) ( options bip32derivs )
//! - [ ] walletlock
//! - [ ] walletpassphrase "passphrase" timeout
//! - [ ] walletpassphrasechange "oldpassphrase" "newpassphrase"
//! - [ ] walletprocesspsbt "psbt" ( sign "sighashtype" bip32derivs )

mod convert;

use serde::{Deserialize, Serialize};

/// Result of the JSON-RPC method `createwallet`.
///
/// > createwallet "wallet_name" ( disable_private_keys )
/// >
/// > Creates and loads a new wallet.
/// >
/// > Arguments:
/// > 1. "wallet_name"          (string, required) The name for the new wallet. If this is a path, the wallet will be created at the path location.
/// > 2. disable_private_keys   (boolean, optional, default: false) Disable the possibility of private keys (only watchonlys are possible in this mode).
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
/// > loadwallet "filename"
/// >
/// > Loads a wallet from a wallet file or directory.
/// > Note that all wallet command-line options used when starting bitcoind will be
/// > applied to the new wallet (eg -zapwallettxes, upgradewallet, rescan, etc).
/// >
/// > Arguments:
/// > 1. "filename"    (string, required) The wallet directory or .dat file.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LoadWallet {
    /// The wallet name if loaded successfully.
    pub name: String,
    /// Warning messages, if any, related to loading the wallet.
    pub warning: String,
}

/// Result of the JSON-RPC method `getnewaddress`.
///
/// > getnewaddress ( "label" "address_type" )
/// >
/// > Returns a new Bitcoin address for receiving payments.
/// > If 'label' is specified, it is added to the address book
/// > so payments received with the address will be associated with 'label'.
/// >
/// > Arguments:
/// > 1. "label"          (string, optional) The label name for the address to be linked to. If not provided, the default label "" is used. It can also be set to the empty string "" to represent the default label. The label does not need to exist, it will be created if there is no label by the given name.
/// > 2. "address_type"   (string, optional) The address type to use. Options are "legacy", "p2sh-segwit", and "bech32". Default is set by -addresstype.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetNewAddress(pub String);

/// Result of the JSON-RPC method `getbalance`.
///
/// > getbalance ( "(dummy)" minconf include_watchonly )
/// >
/// > Returns the total available balance.
/// > The available balance is what the wallet considers currently spendable, and is
/// > thus affected by options which limit spendability such as -spendzeroconfchange.
/// >
/// > Arguments:
/// > 1. (dummy)           (string, optional) Remains for backward compatibility. Must be excluded or set to "*".
/// > 2. minconf           (numeric, optional, default=0) Only include transactions confirmed at least this many times.
/// > 3. include_watchonly (bool, optional, default=false) Also include balance in watch-only addresses (see 'importaddress')
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBalance(pub f64);

/// Result of the JSON-RPC method `sendtoaddress`.
///
/// > sendtoaddress "address" amount ( "comment" "comment_to" subtractfeefromamount replaceable conf_target "estimate_mode")
/// >
/// > Send an amount to a given address.
/// >
/// > Arguments:
/// > 1. "address"            (string, required) The bitcoin address to send to.
/// > 2. "amount"             (numeric or string, required) The amount in BTC to send. eg 0.1
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SendToAddress {
    /// The transaction id.
    pub txid: String,
}
