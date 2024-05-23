// SPDX-License-Identifier: CC0-1.0

//! Convert stdlib (version specific) types to concrete types.
//!
//! This module does the conversion for `v0.17.1` types to the general concrete types.

use std::str::FromStr;

use bitcoin::amount::ParseAmountError;
use bitcoin::{address, hex, Address, Amount, Txid};

use crate::json::v17;
use crate::model;

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
