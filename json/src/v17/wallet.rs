// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core v0.17.1 - wallet.
//!
//! Types for methods found under the `== Wallet ==` section of the API docs.

use std::fmt;
use std::str::FromStr;

use bitcoin::address::NetworkUnchecked;
use bitcoin::amount::ParseAmountError;
use bitcoin::consensus::encode;
use bitcoin::{address, hex, Address, Amount, SignedAmount, Transaction, Txid};
use internals::write_err;
use serde::{Deserialize, Serialize};

use crate::model;

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

impl CreateWallet {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::CreateWallet {
        model::CreateWallet { name: self.name, warnings: vec![self.warning] }
    }

    /// Returns the created wallet name.
    pub fn name(self) -> String { self.into_model().name }
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

impl LoadWallet {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::LoadWallet {
        model::LoadWallet { name: self.name, warnings: vec![self.warning] }
    }

    /// Returns the loaded wallet name.
    pub fn name(self) -> String { self.into_model().name }
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

impl GetNewAddress {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetNewAddress, address::ParseError> {
        let address = Address::from_str(&self.0)?;
        Ok(model::GetNewAddress(address))
    }

    /// Converts json straight to a `bitcoin::Address`.
    pub fn address(self) -> Result<Address<NetworkUnchecked>, address::ParseError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}

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

impl GetBalance {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBalance, ParseAmountError> {
        let amount = Amount::from_btc(self.0)?;
        Ok(model::GetBalance(amount))
    }

    /// Converts json straight to a `bitcoin::Amount`.
    pub fn balance(self) -> Result<Amount, ParseAmountError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}

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
pub struct SendToAddress(String);

impl SendToAddress {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::SendToAddress, hex::HexToArrayError> {
        let txid = self.0.parse::<Txid>()?;
        Ok(model::SendToAddress { txid })
    }

    /// Converts json straight to a `bitcoin::Txid`.
    pub fn txid(self) -> Result<Txid, hex::HexToArrayError> { Ok(self.into_model()?.txid) }
}

/// Result of the JSON-RPC method `gettransaction`.
///
/// > gettransaction "txid" ( include_watchonly )
/// >
/// > Get detailed information about in-wallet transaction `<txid>`
/// >
/// > Arguments:
/// > 1. txid                 (string, required) The transaction id
/// > 2. include_watchonly    (boolean, optional, default=false) Whether to include watch-only addresses in balance calculation and details[]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetTransaction {
    pub amount: f64,
    pub fee: Option<f64>,
    pub confirmations: u32,
    // The docs say there should be two more fields: `blockhash` and `blockindex` but integration
    // test fails if we add them i.e., they are not returned by `v0.17.1`.
    pub txid: String,
    pub time: u64,
    #[serde(rename = "timereceived")]
    pub time_received: u64,
    #[serde(rename = "bip125-replaceable")]
    pub bip125_replaceable: String,
    pub details: Vec<GetTransactionDetail>,
    pub hex: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetTransactionDetail {
    pub address: String,
    pub category: GetTransactionDetailCategory,
    pub amount: f64,
    pub label: Option<String>,
    pub vout: u32,
    pub fee: Option<f64>,
    pub abandoned: Option<bool>,
}

/// Enum to represent the category of a transaction.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GetTransactionDetailCategory {
    Send,
    Receive,
    Generate,
    Immature,
    Orphan,
}

impl GetTransaction {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTransaction, GetTransactionError> {
        use GetTransactionError as E;

        let amount = SignedAmount::from_btc(self.amount).map_err(E::Amount)?;
        // FIMXE: Use combinators.
        let fee = match self.fee {
            None => None,
            Some(f) => Some(SignedAmount::from_btc(f).map_err(E::Fee)?),
        };
        let txid = self.txid.parse::<Txid>().map_err(E::Txid)?;

        let tx = encode::deserialize_hex::<Transaction>(&self.hex).map_err(E::Tx)?;
        let mut details = vec![];
        for detail in self.details {
            let concrete = detail.into_model().map_err(E::Details)?;
            details.push(concrete);
        }

        Ok(model::GetTransaction {
            amount,
            fee,
            confirmations: self.confirmations,
            txid,
            time: self.time,
            time_received: self.time_received,
            bip125_replaceable: self.bip125_replaceable,
            details,
            tx,
        })
    }
}

/// Error when converting a `GetTransaction` type into the model type.
#[derive(Debug)]
pub enum GetTransactionError {
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
    /// Conversion of the `fee` field failed.
    Fee(ParseAmountError),
    /// Conversion of the `txid` field failed.
    Txid(hex::HexToArrayError),
    /// Conversion of the transaction `hex` field failed.
    Tx(encode::FromHexError),
    /// Conversion of the `details` field failed.
    Details(GetTransactionDetailError),
}

impl fmt::Display for GetTransactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetTransactionError as E;

        match *self {
            E::Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
            E::Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
            E::Txid(ref e) => write_err!(f, "conversion of the `txid` field failed"; e),
            E::Tx(ref e) => write_err!(f, "conversion of the `hex` field failed"; e),
            E::Details(ref e) => write_err!(f, "conversion of the `details` field failed"; e),
        }
    }
}

impl std::error::Error for GetTransactionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetTransactionError as E;

        match *self {
            E::Amount(ref e) => Some(e),
            E::Fee(ref e) => Some(e),
            E::Txid(ref e) => Some(e),
            E::Tx(ref e) => Some(e),
            E::Details(ref e) => Some(e),
        }
    }
}

impl GetTransactionDetail {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTransactionDetail, GetTransactionDetailError> {
        use GetTransactionDetailError as E;

        let address = Address::from_str(&self.address).map_err(E::Address)?;
        let amount = SignedAmount::from_btc(self.amount).map_err(E::Amount)?;
        // FIMXE: Use combinators.
        let fee = match self.fee {
            None => None,
            Some(f) => Some(SignedAmount::from_btc(f).map_err(E::Fee)?),
        };

        Ok(model::GetTransactionDetail {
            address,
            category: self.category.into_model(),
            amount,
            label: self.label,
            vout: self.vout,
            fee,
            abandoned: self.abandoned,
        })
    }
}

/// Error when converting to a `v22::GetTransactionDetail` type to a `concrete` type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetTransactionDetailError {
    /// Conversion of the `address` field failed.
    Address(address::ParseError),
    /// Conversion of the `fee` field failed.
    Fee(ParseAmountError),
    /// Conversion of the `amount` field failed.
    Amount(ParseAmountError),
}

impl fmt::Display for GetTransactionDetailError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetTransactionDetailError::*;

        match *self {
            Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
            Fee(ref e) => write_err!(f, "conversion of the `fee` field failed"; e),
            Amount(ref e) => write_err!(f, "conversion of the `amount` field failed"; e),
        }
    }
}

impl std::error::Error for GetTransactionDetailError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetTransactionDetailError as E;

        match *self {
            E::Address(ref e) => Some(e),
            E::Fee(ref e) => Some(e),
            E::Amount(ref e) => Some(e),
        }
    }
}

impl GetTransactionDetailCategory {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::GetTransactionDetailCategory {
        use GetTransactionDetailCategory::*;

        match self {
            Send => model::GetTransactionDetailCategory::Send,
            Receive => model::GetTransactionDetailCategory::Receive,
            Generate => model::GetTransactionDetailCategory::Generate,
            Immature => model::GetTransactionDetailCategory::Immature,
            Orphan => model::GetTransactionDetailCategory::Orphan,
        }
    }
}
