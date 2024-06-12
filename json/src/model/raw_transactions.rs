// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the `== Rawtransactions ==` section of the API docs.
//!
//! These structs model the types returned by the JSON-RPC API but have concrete types
//! and are not specific to a specific version of Bitcoin Core.

use bitcoin::Txid;
use serde::{Deserialize, Serialize};

/// Models the result of JSON-RPC method `sendrawtransaction`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SendRawTransaction(pub Txid);
