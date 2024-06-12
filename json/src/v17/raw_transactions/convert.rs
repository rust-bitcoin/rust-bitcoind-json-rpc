// SPDX-License-Identifier: CC0-1.0

//! Convert stdlib (version specific) types to concrete types.
//!
//! This module does the conversion for `v0.17.1` types to the general concrete types.

use bitcoin::{hex, Txid};

use crate::{model, v17};

impl TryFrom<v17::SendRawTransaction> for model::SendRawTransaction {
    type Error = hex::HexToArrayError;

    fn try_from(json: v17::SendRawTransaction) -> Result<Self, Self::Error> {
        let txid = json.0.parse::<Txid>()?;
        Ok(Self(txid))
    }
}
