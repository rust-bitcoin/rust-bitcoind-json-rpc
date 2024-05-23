// SPDX-License-Identifier: CC0-1.0

//! Convert stdlib (version specific) types to concrete types.
//!
//! This module does the conversion for `v26.1` types to the general concrete types.

use crate::json::v26;
use crate::model;

impl From<v26::CreateWallet> for model::CreateWallet {
    fn from(json: v26::CreateWallet) -> Self {
        Self { name: json.name, warnings: json.warnings.unwrap_or_default() }
    }
}
