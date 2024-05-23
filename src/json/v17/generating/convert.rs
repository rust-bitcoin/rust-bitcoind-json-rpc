// SPDX-License-Identifier: CC0-1.0

//! Convert stdlib (version specific) types to concrete types.
//!
//! This module does the conversion for `v0.17.1` types to the general concrete types.

use bitcoin::{hex, BlockHash};

use crate::json::v17;
use crate::model;

impl TryFrom<v17::GenerateToAddress> for model::GenerateToAddress {
    type Error = hex::HexToArrayError;

    fn try_from(json: v17::GenerateToAddress) -> Result<Self, Self::Error> {
        // FIXME: Use combinators.
        let mut v = vec![];
        for s in json.0.iter() {
            let hash = s.parse::<BlockHash>()?;
            v.push(hash);
        }
        Ok(Self(v))
    }
}
