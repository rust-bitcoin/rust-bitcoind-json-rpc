// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core v0.17.1 - raw transactions.
//!
//! Types for methods found under the `== Rawtransactions ==` section of the API docs.

use bitcoin::{hex, Txid};
use serde::{Deserialize, Serialize};

use crate::model;

/// Result of JSON-RPC method `sendrawtransaction`.
///
/// > sendrawtransaction "hexstring" ( allowhighfees )
/// >
/// > Submits raw transaction (serialized, hex-encoded) to local node and network.
/// >
/// > Also see createrawtransaction and signrawtransactionwithkey calls.
/// >
/// > Arguments:
/// > 1. hexstring        (string, required) The hex string of the raw transaction
/// > 2. allowhighfees    (boolean, optional, default=false) Allow high fees
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendRawTransaction(pub String); // The hex encoded txid.

impl SendRawTransaction {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::SendRawTransaction, hex::HexToArrayError> {
        let txid = self.0.parse::<Txid>()?;
        Ok(model::SendRawTransaction(txid))
    }

    /// Converts json straight to a `bitcoin::Txid`.
    pub fn txid(self) -> Result<Txid, hex::HexToArrayError> {
        let model = self.into_model()?;
        Ok(model.0)
    }
}
