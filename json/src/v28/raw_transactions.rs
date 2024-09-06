// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core v28.0 - raw transactions.
//!
//! Types for methods found under the `== Rawtransactions ==` section of the API docs.

use std::collections::HashMap;

use bitcoin::{Amount, FeeRate, Txid, Wtxid};
use serde::{Deserialize, Serialize};

/// Models the result of JSON-RPC method `submitpackage`.
//submitpackage ["rawtx",...] ( maxfeerate maxburnamount )
//
//Submit a package of raw transactions (serialized, hex-encoded) to local node.
//The package will be validated according to consensus and mempool policy rules. If any transaction passes, it will be accepted to mempool.
//This RPC is experimental and the interface may be unstable. Refer to doc/policy/packages.md for documentation on package policies.
//Warning: successful submission does not mean the transactions will propagate throughout the network.
//
//Arguments:
//1. package          (json array, required) An array of raw transactions.
//                    The package must solely consist of a child and its parents. None of the parents may depend on each other.
//                    The package must be topologically sorted, with the child being the last element in the array.
//     [
//       "rawtx",     (string)
//       ...
//     ]
//2. maxfeerate       (numeric or string, optional, default="0.10") Reject transactions whose fee rate is higher than the specified value, expressed in BTC/kvB.
//                    Fee rates larger than 1BTC/kvB are rejected.
//                    Set to 0 to accept any fee rate.
//3. maxburnamount    (numeric or string, optional, default="0.00") Reject transactions with provably unspendable outputs (e.g. 'datacarrier' outputs that use the OP_RETURN opcode) greater than the specified value, expressed in BTC.
//                    If burning funds through unspendable outputs is desired, increase this value.
//                    This check is based on heuristics and does not guarantee spendability of outputs.
//
//
//Result:
//{                                   (json object)
//  "package_msg" : "str",            (string) The transaction package result message. "success" indicates all transactions were accepted into or are already in the mempool.
//  "tx-results" : {                  (json object) transaction results keyed by wtxid
//    "wtxid" : {                     (json object) transaction wtxid
//      "txid" : "hex",               (string) The transaction hash in hex
//      "other-wtxid" : "hex",        (string, optional) The wtxid of a different transaction with the same txid but different witness found in the mempool. This means the submitted transaction was ignored.
//      "vsize" : n,                  (numeric, optional) Sigops-adjusted virtual transaction size.
//      "fees" : {                    (json object, optional) Transaction fees
//        "base" : n,                 (numeric) transaction fee in BTC
//        "effective-feerate" : n,    (numeric, optional) if the transaction was not already in the mempool, the effective feerate in BTC per KvB. For example, the package feerate and/or feerate with modified fees from prioritisetransaction.
//        "effective-includes" : [    (json array, optional) if effective-feerate is provided, the wtxids of the transactions whose fees and vsizes are included in effective-feerate.
//          "hex",                    (string) transaction wtxid in hex
//          ...
//        ]
//      },
//      "error" : "str"               (string, optional) The transaction error string, if it was rejected by the mempool
//    },
//    ...
//  },
//  "replaced-transactions" : [       (json array, optional) List of txids of replaced transactions
//    "hex",                          (string) The transaction id
//    ...
//  ]
//}
//
//Examples:
//> curl --user myusername --data-binary '{"jsonrpc": "2.0", "id": "curltest", "method": "submitpackage", "params": [["rawtx1", "rawtx2"]]}' -H 'content-type: application/json' http://127.0.0.1:8332/
//> bitcoin-cli submitpackage '["rawtx1", "rawtx2"]'
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubmitPackage {
    /// The transaction package result message. "success" indicates all transactions were accepted into or are already in the mempool.
    pub package_msg: String,
    /// Transaction results keyed by [`Wtxid`].
    #[serde(rename = "tx-results")]
    pub tx_results: HashMap<Wtxid, SubmitPackageTxResult>,
    /// List of txids of replaced transactions.
    #[serde(rename = "replaced-transactions")]
    pub replaced_transactions: Vec<Txid>,
}

/// Models the per-transaction result included in the JSON-RPC method `submitpackage`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubmitPackageTxResult {
    /// The transaction id.
    pub txid: Txid,
    /// The [`Wtxid`] of a different transaction with the same [`Txid`] but different witness found in the mempool.
    ///
    /// If set, this means the submitted transaction was ignored.
    #[serde(rename = "other-wtxid")]
    pub other_wtxid: Option<Wtxid>,
    /// Sigops-adjusted virtual transaction size.
    pub vsize: Option<usize>,
    /// Transaction fees.
    pub fees: Option<SubmitPackageTxResultFees>,
    /// The transaction error string, if it was rejected by the mempool
    pub error: Option<String>,
}

/// Models the fees included in the per-transaction result of the JSON-RPC method `submitpackage`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct SubmitPackageTxResultFees {
    /// Transaction fee.
    #[serde(rename = "base")]
    pub base_fee: Amount,
    /// The effective feerate.
    ///
    /// Will be `None` if the transaction was already in the mempool.
    ///
    /// For example, the package feerate and/or feerate with modified fees from the `prioritisetransaction` JSON-RPC method.
    #[serde(rename = "effective-feerate")]
    pub effective_feerate: Option<FeeRate>,
    /// If [`Self::effective_feerate`] is provided, this holds the [`Wtxid`]s of the transactions
    /// whose fees and vsizes are included in effective-feerate.
    #[serde(rename = "effective-includes")]
    pub effective_includes: Vec<Wtxid>,
}
