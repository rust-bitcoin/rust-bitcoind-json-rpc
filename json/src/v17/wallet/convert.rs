// SPDX-License-Identifier: CC0-1.0

//! Convert stdlib (version specific) types to concrete types.
//!
//! This module does the conversion for `v0.17.1` types to the general concrete types.

use std::fmt;
use std::str::FromStr;

use bitcoin::amount::ParseAmountError;
use bitcoin::consensus::encode;
use bitcoin::{address, hex, Address, Amount, SignedAmount, Transaction, Txid};
use internals::write_err;

use crate::{model, v17};

impl From<v17::CreateWallet> for model::CreateWallet {
    fn from(json: v17::CreateWallet) -> Self {
        Self { name: json.name, warnings: vec![json.warning] }
    }
}

impl From<v17::LoadWallet> for model::LoadWallet {
    fn from(json: v17::LoadWallet) -> Self {
        Self { name: json.name, warnings: vec![json.warning] }
    }
}

impl TryFrom<v17::SendToAddress> for model::SendToAddress {
    type Error = hex::HexToArrayError;

    fn try_from(json: v17::SendToAddress) -> Result<Self, Self::Error> {
        let txid = json.txid.parse::<Txid>()?;
        Ok(Self {
            txid,
            // FIXME: Is this acceptable?
            fee_reason: "".to_string(),
        })
    }
}

impl TryFrom<v17::GetNewAddress> for model::GetNewAddress {
    type Error = address::ParseError;

    fn try_from(json: v17::GetNewAddress) -> Result<Self, Self::Error> {
        let address = Address::from_str(&json.0)?;
        Ok(Self(address))
    }
}

impl TryFrom<v17::GetBalance> for model::GetBalance {
    type Error = ParseAmountError;

    fn try_from(json: v17::GetBalance) -> Result<Self, Self::Error> {
        let amount = Amount::from_btc(json.0)?;
        Ok(Self(amount))
    }
}

impl TryFrom<v17::GetTransaction> for model::GetTransaction {
    type Error = GetTransactionError;

    fn try_from(json: v17::GetTransaction) -> Result<Self, Self::Error> {
        use GetTransactionError as E;

        let amount = Amount::from_btc(json.amount).map_err(E::Amount)?;
        // FIMXE: Use combinators.
        let fee = match json.fee {
            None => None,
            Some(f) => Some(SignedAmount::from_btc(f).map_err(E::Fee)?),
        };
        let txid = json.txid.parse::<Txid>().map_err(E::Txid)?;

        let tx = encode::deserialize_hex::<Transaction>(&json.hex).map_err(E::Tx)?;
        let mut details = vec![];
        for detail in json.details {
            let concrete = detail.try_into().map_err(E::Details)?;
            details.push(concrete);
        }

        Ok(Self {
            amount,
            fee,
            confirmations: json.confirmations,
            txid,
            time: json.time,
            time_received: json.time_received,
            bip125_replaceable: json.bip125_replaceable,
            details,
            tx,
        })
    }
}

/// Error when converting to a `v22::GetBlockchainInfo` type to a `concrete` type.
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

impl TryFrom<v17::GetTransactionDetail> for model::GetTransactionDetail {
    type Error = GetTransactionDetailError;

    fn try_from(json: v17::GetTransactionDetail) -> Result<Self, Self::Error> {
        use GetTransactionDetailError as E;

        let address = Address::from_str(&json.address).map_err(E::Address)?;
        let amount = Amount::from_btc(json.amount).map_err(E::Amount)?;
        // FIMXE: Use combinators.
        let fee = match json.fee {
            None => None,
            Some(f) => Some(SignedAmount::from_btc(f).map_err(E::Fee)?),
        };

        Ok(Self {
            address,
            category: json.category.into(),
            amount,
            label: json.label,
            vout: json.vout,
            fee,
            abandoned: json.abandoned,
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

impl From<v17::GetTransactionDetailCategory> for model::GetTransactionDetailCategory {
    fn from(json: v17::GetTransactionDetailCategory) -> Self {
        use v17::GetTransactionDetailCategory::*;

        match json {
            Send => model::GetTransactionDetailCategory::Send,
            Receive => model::GetTransactionDetailCategory::Receive,
            Generate => model::GetTransactionDetailCategory::Generate,
            Immature => model::GetTransactionDetailCategory::Immature,
            Orphan => model::GetTransactionDetailCategory::Orphan,
        }
    }
}
