// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the blockchain section of the API docs.
//!
//! The JSON-RPC API for Bitcoin Core v26.1
//!
//! == Blockchain ==
//!
//! - [ ] dumptxoutset "path"
//! - [x] getbestblockhash
//! - [ ] getblock "blockhash" ( verbosity )
//! - [x] getblockchaininfo
//! - [ ] getblockcount
//! - [ ] getblockfilter "blockhash" ( "filtertype" )
//! - [ ] getblockfrompeer "blockhash" peer_id
//! - [ ] getblockhash height
//! - [ ] getblockheader "blockhash" ( verbose )
//! - [ ] getblockstats hash_or_height ( stats )
//! - [ ] getchainstates
//! - [ ] getchaintips
//! - [ ] getchaintxstats ( nblocks "blockhash" )
//! - [ ] getdeploymentinfo ( "blockhash" )
//! - [ ] getdifficulty
//! - [ ] getmempoolancestors "txid" ( verbose )
//! - [ ] getmempooldescendants "txid" ( verbose )
//! - [ ] getmempoolentry "txid"
//! - [ ] getmempoolinfo
//! - [ ] getrawmempool ( verbose mempool_sequence )
//! - [ ] gettxout "txid" n ( include_mempool )
//! - [ ] gettxoutproof ["txid",...] ( "blockhash" )
//! - [ ] gettxoutsetinfo ( "hash_type" hash_or_height use_index )
//! - [ ] gettxspendingprevout [{"txid":"hex","vout":n},...]
//! - [ ] importmempool "filepath" ( options )
//! - [ ] loadtxoutset "path"
//! - [ ] preciousblock "blockhash"
//! - [ ] pruneblockchain height
//! - [ ] savemempool
//! - [ ] scanblocks "action" ( [scanobjects,...] start_height stop_height "filtertype" options )
//! - [ ] scantxoutset "action" ( [scanobjects,...] )
//! - [ ] verifychain ( checklevel nblocks )
//! - [ ] verifytxoutproof "proof"

pub use crate::json::v22::blockchain::{
    GetBestBlockHash, GetBlockVerbosityOne, GetBlockVerbosityZero, GetBlockchainInfo, GetTxOut,
};
