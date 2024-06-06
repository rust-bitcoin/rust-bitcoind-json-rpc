// SPDX-License-Identifier: CC0-1.0

//! Convert stdlib (version specific) types to concrete types.
//!
//! This module does the conversion for `v0.19.1` types to the general concrete types.

use bitcoin::amount::ParseAmountError;
use bitcoin::Amount;

use crate::{model, v19};

impl TryFrom<v19::GetBalances> for model::GetBalances {
    type Error = ParseAmountError;

    fn try_from(json: v19::GetBalances) -> Result<Self, Self::Error> {
        let mine = json.mine.try_into()?;
        // FIXME: Use combinators instead of matching like a noob.
        let watch_only = match json.watch_only {
            Some(watch_only) => Some(watch_only.try_into()?),
            None => None,
        };

        Ok(Self { mine, watch_only })
    }
}

impl TryFrom<v19::GetBalancesMine> for model::GetBalancesMine {
    type Error = ParseAmountError;

    fn try_from(json: v19::GetBalancesMine) -> Result<Self, Self::Error> {
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

impl TryFrom<v19::GetBalancesWatchOnly> for model::GetBalancesWatchOnly {
    type Error = ParseAmountError;

    fn try_from(json: v19::GetBalancesWatchOnly) -> Result<Self, Self::Error> {
        let trusted = Amount::from_btc(json.trusted)?;
        let untrusted_pending = Amount::from_btc(json.untrusted_pending)?;
        let immature = Amount::from_btc(json.immature)?;

        Ok(Self { trusted, untrusted_pending, immature })
    }
}
