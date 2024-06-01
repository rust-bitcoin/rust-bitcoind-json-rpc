// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the `== Generating ==` section of the API docs.
//!
//! These structs model the types returned by the JSON-RPC API but have concrete types
//! and are not specific to a specific version of Bitcoin Core.

use bitcoin::BlockHash;
use serde::{Deserialize, Serialize};

/// Model of the result of JSON-RPC method `generatetoaddress`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GenerateToAddress(pub Vec<BlockHash>);
