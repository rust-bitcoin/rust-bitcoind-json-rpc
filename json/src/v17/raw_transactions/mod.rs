// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core v0.17.1 - raw transactions.
//!
//! Types for methods found under the `== Rawtransactions ==` section of the API docs.

mod convert;

use serde::{Deserialize, Serialize};

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
