// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the blockchain section of the API docs.
//!
//! The JSON-RPC API for Bitcoin Core v0.17.1
//!
//! == Blockchain ==
//!
//! - [ ] getbestblockhash
//! - [ ] getblock "blockhash" ( verbosity )
//! - [ ] getblockchaininfo
//! - [ ] getblockcount
//! - [ ] getblockhash height
//! - [ ] getblockheader "hash" ( verbose )
//! - [ ] getblockstats hash_or_height ( stats )
//! - [ ] getchaintips
//! - [ ] getchaintxstats ( nblocks blockhash )
//! - [ ] getdifficulty
//! - [ ] getmempoolancestors txid (verbose)
//! - [ ] getmempooldescendants txid (verbose)
//! - [ ] getmempoolentry txid
//! - [ ] getmempoolinfo
//! - [ ] getrawmempool ( verbose )
//! - [ ] gettxout "txid" n ( include_mempool )
//! - [ ] gettxoutproof ["txid",...] ( blockhash )
//! - [ ] gettxoutsetinfo
//! - [ ] preciousblock "blockhash"
//! - [ ] pruneblockchain
//! - [ ] savemempool
//! - [ ] scantxoutset <action> ( <scanobjects> )
//! - [ ] verifychain ( checklevel nblocks )
//! - [ ] verifytxoutproof "proof"

mod convert;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Result of JSON-RPC method `getbestblockhash`.
///
/// > getbestblockhash
/// >
/// > Returns the hash of the best (tip) block in the most-work fully-validated chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBestBlockHash(pub String);

/// Result of JSON-RPC method `getblockchaininfo`.
///
/// Method call: `getblockchaininfo`
///
/// > Returns an object containing various state info regarding blockchain processing.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockchainInfo {
    /// Current network name as defined in BIP70 (main, test, signet, regtest).
    pub chain: String,
    /// The current number of blocks processed in the server.
    pub blocks: u64,
    /// The current number of headers we have validated.
    pub headers: u64,
    /// The hash of the currently best block.
    #[serde(rename = "bestblockhash")]
    pub best_block_hash: String,
    /// The current difficulty.
    pub difficulty: f64,
    /// Median time for the current best block.
    #[serde(rename = "mediantime")]
    pub median_time: u64,
    /// Estimate of verification progress (between 0 and 1).
    #[serde(rename = "verificationprogress")]
    pub verification_progress: f64,
    /// Estimate of whether this node is in Initial Block Download (IBD) mode.
    #[serde(rename = "initialblockdownload")]
    pub initial_block_download: bool,
    /// Total amount of work in active chain, in hexadecimal.
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// The estimated size of the block and undo files on disk.
    pub size_on_disk: u64,
    /// If the blocks are subject to pruning.
    pub pruned: bool,
    /// Lowest-height complete block stored (only present if pruning is enabled).
    #[serde(rename = "pruneheight")]
    pub prune_height: Option<u64>,
    /// Whether automatic pruning is enabled (only present if pruning is enabled).
    pub automatic_pruning: Option<bool>,
    /// The target size used by pruning (only present if automatic pruning is enabled).
    pub prune_target_size: Option<u64>,
    /// Status of softforks in progress.
    pub softforks: Vec<Softfork>,
    /// Status of BIP-9 softforks in progress, maps softfork name -> [`Softfork`].
    pub bip9_softforks: BTreeMap<String, Bip9Softfork>,
    /// Any network and blockchain warnings.
    pub warnings: String,
}

/// Status of softfork.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Softfork {
    /// Name of softfork.
    id: String,
    /// Block version.
    version: usize,
    /// Progress toward rejecting pre-softfork blocks.
    reject: SoftforkReject,
}

/// Progress toward rejecting pre-softfork blocks.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct SoftforkReject {
    /// `true` if threshold reached.
    status: bool,
}

/// Status of BIP-9 softforksin progress.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct Bip9Softfork {
    /// One of "defined", "started", "locked_in", "active", "failed".
    pub status: Bip9SoftforkStatus,
    /// The bit (0-28) in the block version field used to signal this softfork (only for "started" status).
    pub bit: Option<u8>,
    /// The minimum median time past of a block at which the bit gains its meaning.
    #[serde(rename = "startTime")]
    pub start_time: i64,
    /// The median time past of a block at which the deployment is considered failed if not yet locked in.
    pub timeout: u64,
    /// Height of the first block to which the status applies.
    pub since: u32,
}

/// BIP-9 softfork status: one of "defined", "started", "locked_in", "active", "failed".
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bip9SoftforkStatus {
    /// BIP-9 softfork status "defined".
    Defined,
    /// BIP-9 softfork status "started".
    Started,
    /// BIP-9 softfork status "locked_in".
    LockedIn,
    /// BIP-9 softfork status "active".
    Active,
    /// BIP-9 softfork status "failed".
    Failed,
}

/// Result of JSON-RPC method `getblock` with verbosity set to 0.
///
/// A string that is serialized, hex-encoded data for block 'hash'.
///
/// Method call: `getblock "blockhash" ( verbosity )`
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct GetBlockVerbosityZero(pub String);

/// Result of JSON-RPC method `getblock` with verbosity set to 1.
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct GetBlockVerbosityOne {
    /// The block hash (same as provided) in RPC call.
    pub hash: String,
    /// The number of confirmations, or -1 if the block is not on the main chain.
    pub confirmations: i32,
    /// The block size.
    pub size: usize,
    /// The block size excluding witness data.
    pub strippedsize: Option<usize>,
    /// The block weight as defined in BIP-141.
    pub weight: u64,
    /// The block height or index.
    pub height: usize,
    /// The block version.
    pub version: i32,
    /// The block version formatted in hexadecimal.
    #[serde(rename = "versionHex")]
    pub version_hex: String,
    /// The merkle root
    pub merkleroot: String,
    /// The transaction ids
    pub tx: Vec<String>,
    /// The block time expressed in UNIX epoch time.
    pub time: usize,
    /// The median block time expressed in UNIX epoch time.
    #[serde(rename = "mediantime")]
    pub median_time: Option<usize>,
    /// The nonce
    pub nonce: u32,
    /// The bits.
    pub bits: String,
    /// The difficulty.
    pub difficulty: f64,
    /// Expected number of hashes required to produce the chain up to this block (in hex).
    #[serde(rename = "chainwork")]
    pub chain_work: String,
    /// The number of transactions in the block.
    #[serde(rename = "nTx")]
    pub n_tx: u32,
    /// The hash of the previous block (if available).
    #[serde(rename = "previousblockhash")]
    pub previous_block_hash: Option<String>,
    /// The hash of the next block (if available).
    #[serde(rename = "nextblockhash")]
    pub next_block_hash: Option<String>,
}

/// Result of JSON-RPC method `gettxout`.
///
/// > gettxout "txid" n ( include_mempool )
/// >
/// > Returns details about an unspent transaction output.
/// >
/// > Arguments:
/// > 1. txid               (string, required) The transaction id
/// > 2. n                  (numeric, required) vout number
/// > 3. include_mempool    (boolean, optional, default=true) Whether to include the mempool. Note that an unspent output that is spent in the mempool won't appear.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetTxOut {
    /// The hash of the block at the tip of the chain.
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// The number of confirmations.
    pub confirmations: u64,
    /// The transaction value in BTC.
    pub value: u64,
    /// The script pubkey.
    #[serde(rename = "scriptPubkey")]
    pub script_pubkey: ScriptPubkey,
    /// Coinbase or not.
    pub coinbase: bool,
}

/// A script pubkey.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ScriptPubkey {
    /// Script assembyl.
    pub asm: String,
    /// Script hex.
    pub hex: String,
    // TODO: Add support for deprecatedrpc=addresses
    // #[serde(rename = "reqSigs")]
    // pub req_sigs: u64,
    /// The type, eg pubkeyhash
    #[serde(rename = "type")]
    pub type_: String,
    /// bitcoin address
    pub address: String,
    // TODO: Add support for deprecatedrpc=addresses
    // pub addressess: Vec<String>,
}
