// SPDX-License-Identifier: CC0-1.0

//! Convert stdlib (version specific) types to concrete types.
//!
//! This module does the conversion for `v22.1` types to the general concrete types.

use bitcoin::{hex, Txid};

use crate::{model, v22};

impl From<v22::UnloadWallet> for model::UnloadWallet {
    fn from(json: v22::UnloadWallet) -> Self { Self { warnings: vec![json.warning] } }
}

impl TryFrom<v22::SendToAddress> for model::SendToAddress {
    type Error = hex::HexToArrayError;

    fn try_from(json: v22::SendToAddress) -> Result<Self, Self::Error> {
        let txid = json.txid.parse::<Txid>()?;
        Ok(Self { txid, fee_reason: json.fee_reason })
    }
}
