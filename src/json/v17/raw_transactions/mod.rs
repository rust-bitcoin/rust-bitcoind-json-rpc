// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the wallet section of the API docs.
//!
//! The JSON-RPC API for Bitcoin Core v0.17.1:
//!
//!  == Rawtransactions ==
//!
//! - [ ] combinepsbt ["psbt",...]
//! - [ ] combinerawtransaction ["hexstring",...]
//! - [ ] converttopsbt "hexstring" ( permitsigdata iswitness )
//! - [ ] createpsbt [{"txid":"id","vout":n},...] [{"address":amount},{"data":"hex"},...] ( locktime ) ( replaceable )
//! - [ ] createrawtransaction [{"txid":"id","vout":n},...] [{"address":amount},{"data":"hex"},...] ( locktime ) ( replaceable )
//! - [ ] decodepsbt "psbt"
//! - [ ] decoderawtransaction "hexstring" ( iswitness )
//! - [ ] decodescript "hexstring"
//! - [ ] finalizepsbt "psbt" ( extract )
//! - [ ] fundrawtransaction "hexstring" ( options iswitness )
//! - [ ] getrawtransaction "txid" ( verbose "blockhash" )
//! - [ ] sendrawtransaction "hexstring" ( allowhighfees )
//! - [ ] signrawtransaction "hexstring" ( [{"txid":"id","vout":n,"scriptPubKey":"hex","redeemScript":"hex"},...] ["privatekey1",...] sighashtype )
//! - [ ] signrawtransactionwithkey "hexstring" ["privatekey1",...] ( [{"txid":"id","vout":n,"scriptPubKey":"hex","redeemScript":"hex"},...] sighashtype )
//! - [ ] testmempoolaccept ["rawtxs"] ( allowhighfees )
