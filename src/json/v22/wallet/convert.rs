// SPDX-License-Identifier: CC0-1.0

//! Convert stdlib (version specific) types to concrete types.
//!
//! This module does the conversion for `v22.1` types to the general concrete types.

use bitcoin::amount::ParseAmountError;
use bitcoin::{hex, Amount, Txid};

use crate::json::v22;
use crate::model;

impl From<v22::CreateWallet> for model::CreateWallet {
    fn from(json: v22::CreateWallet) -> Self {
        Self { name: json.name, warnings: vec![json.warning] }
    }
}

impl From<v22::LoadWallet> for model::LoadWallet {
    fn from(json: v22::LoadWallet) -> Self {
        Self { name: json.name, warnings: vec![json.warning] }
    }
}

impl From<v22::UnloadWallet> for model::UnloadWallet {
    fn from(json: v22::UnloadWallet) -> Self { Self { warnings: vec![json.warning] } }
}

impl TryFrom<v22::GetBalance> for model::GetBalance {
    type Error = ParseAmountError;

    fn try_from(json: v22::GetBalance) -> Result<Self, Self::Error> {
        let amount = Amount::from_btc(json.0)?;
        Ok(Self(amount))
    }
}

impl TryFrom<v22::GetBalances> for model::GetBalances {
    type Error = ParseAmountError;

    fn try_from(json: v22::GetBalances) -> Result<Self, Self::Error> {
        let mine = json.mine.try_into()?;
        // FIXME: Use combinators instead of matching like a noob.
        let watch_only = match json.watch_only {
            Some(watch_only) => Some(watch_only.try_into()?),
            None => None,
        };

        Ok(Self { mine, watch_only })
    }
}

impl TryFrom<v22::GetBalancesMine> for model::GetBalancesMine {
    type Error = ParseAmountError;

    fn try_from(json: v22::GetBalancesMine) -> Result<Self, Self::Error> {
        let trusted = Amount::from_btc(json.trusted)?;
        let untrusted_pending = Amount::from_btc(json.untrusted_pending)?;
        let immature = Amount::from_btc(json.immature)?;
        // FIXME: Use combinators instead of matching like a noob.
        let used = match json.used {
            Some(used) => Some(Amount::from_btc(used)?),
            None => None,
        };

        Ok(Self { trusted, untrusted_pending, immature, used })
    }
}

impl TryFrom<v22::GetBalancesWatchOnly> for model::GetBalancesWatchOnly {
    type Error = ParseAmountError;

    fn try_from(json: v22::GetBalancesWatchOnly) -> Result<Self, Self::Error> {
        let trusted = Amount::from_btc(json.trusted)?;
        let untrusted_pending = Amount::from_btc(json.untrusted_pending)?;
        let immature = Amount::from_btc(json.immature)?;

        Ok(Self { trusted, untrusted_pending, immature })
    }
}

impl TryFrom<v22::SendToAddress> for model::SendToAddress {
    type Error = hex::HexToArrayError;

    fn try_from(json: v22::SendToAddress) -> Result<Self, Self::Error> {
        let txid = json.txid.parse::<Txid>()?;
        Ok(Self { txid, fee_reason: json.fee_reason })
    }
}
