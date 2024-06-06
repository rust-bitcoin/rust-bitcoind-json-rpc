// SPDX-License-Identifier: CC0-1.0

//! Types for methods found under the `== Blockchain ==` section of the API docs.
//!
//! These structs model the types returned by the JSON-RPC API but have concrete types
//! and are not specific to a specific version of Bitcoin Core.

use std::collections::BTreeMap;

use bitcoin::address::NetworkUnchecked;
use bitcoin::{block, Address, Block, BlockHash, CompactTarget, Network, TxOut, Weight, Work};
use serde::{Deserialize, Serialize};

/// Models the result of JSON-RPC method `getblockchaininfo`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockchainInfo {
    /// Current network name as defined in BIP70 (main, test, signet, regtest).
    pub chain: Network,
    /// The current number of blocks processed in the server.
    pub blocks: u64,
    /// The current number of headers we have validated.
    pub headers: u64,
    /// The hash of the currently best block.
    pub best_block_hash: BlockHash,
    /// The current difficulty.
    pub difficulty: f64,
    /// Median time for the current best block.
    pub median_time: u64,
    /// Estimate of verification progress (between 0 and 1).
    pub verification_progress: f64,
    /// Estimate of whether this node is in Initial Block Download (IBD) mode.
    pub initial_block_download: bool,
    /// Total amount of work in active chain.
    pub chain_work: Work,
    /// The estimated size of the block and undo files on disk.
    pub size_on_disk: u64,
    /// If the blocks are subject to pruning.
    pub pruned: bool,
    /// Lowest-height complete block stored (only present if pruning is enabled)
    pub prune_height: Option<u64>,
    /// Whether automatic pruning is enabled (only present if pruning is enabled).
    pub automatic_pruning: Option<bool>,
    /// The target size used by pruning (only present if automatic pruning is enabled).
    pub prune_target_size: Option<u64>,
    /// Status of softforks in progress, maps softfork name -> [`Softfork`].
    pub softforks: BTreeMap<String, Softfork>,
    /// Any network and blockchain warnings.
    pub warnings: String,
}

/// Status of softfork.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Softfork {
    /// The [`SoftforkType`]: one of "burried", "bip9".
    #[serde(rename = "type")]
    pub type_: SoftforkType,
    /// The status of bip9 softforks (only for "bip9" type).
    pub bip9: Option<Bip9SoftforkInfo>,
    ///  Height of the first block which the rules are or will be enforced (only for "buried" type, or "bip9" type with "active" status).
    pub height: Option<u64>,
    /// `true` if the rules are enforced for the mempool and the next block.
    pub active: bool,
}

/// The softfork type: one of "burried", "bip9".
#[derive(Copy, Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SoftforkType {
    /// Softfork is "burried" (as defined in [BIP-90]).
    ///
    /// [BIP-90] <https://github.com/bitcoin/bips/blob/master/bip-0090.mediawiki>
    Buried,
    /// Softfork is "bip9" (see [BIP-9]).
    ///
    /// [BIP-9] <https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki>
    Bip9,
}

/// Status of BIP-9 softforks.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct Bip9SoftforkInfo {
    /// One of "defined", "started", "locked_in", "active", "failed".
    pub status: Bip9SoftforkStatus,
    /// The bit (0-28) in the block version field used to signal this softfork (only for "started" status).
    pub bit: Option<u8>,
    /// The minimum median time past of a block at which the bit gains its meaning.
    pub start_time: i64,
    /// The median time past of a block at which the deployment is considered failed if not yet locked in.
    pub timeout: u64,
    /// Height of the first block to which the status applies.
    pub since: u32,
    /// Numeric statistics about BIP-9 signalling for a softfork (only for "started" status).
    pub statistics: Option<Bip9SoftforkStatistics>,
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

/// Statistics for a BIP-9 softfork.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub struct Bip9SoftforkStatistics {
    /// The length in blocks of the BIP9 signalling period.
    pub period: u32,
    /// The number of blocks with the version bit set required to activate the feature.
    pub threshold: Option<u32>,
    /// The number of blocks elapsed since the beginning of the current period.
    pub elapsed: u32,
    /// The number of blocks with the version bit set in the current period.
    pub count: u32,
    /// `false` if there are not enough blocks left in this period to pass activation threshold.
    pub possible: Option<bool>,
}

/// Models the result of JSON-RPC method `getblock` with verbosity set to 0.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBlockVerbosityZero(pub Block);

/// Models the result of JSON-RPC method `getblock` with verbosity set to 1.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBlockVerbosityOne {
    /// The block hash (same as provided) in RPC call.
    pub hash: BlockHash,
    /// The number of confirmations, or -1 if the block is not on the main chain.
    pub confirmations: i32,
    /// The block size.
    pub size: usize,
    /// The block size excluding witness data.
    pub strippedsize: Option<usize>, // Weight?
    /// The block weight as defined in BIP-141.
    pub weight: Weight,
    /// The block height or index.
    pub height: usize,
    /// The block version.
    pub version: block::Version,
    /// The block version formatted in hexadecimal.
    pub version_hex: String,
    /// The merkle root
    pub merkleroot: String,
    /// The transaction ids
    pub tx: Vec<String>,
    /// The block time expressed in UNIX epoch time.
    pub time: usize,
    /// The median block time expressed in UNIX epoch time.
    pub median_time: Option<usize>,
    /// The nonce
    pub nonce: u32,
    /// The bits.
    pub bits: CompactTarget,
    /// The difficulty.
    pub difficulty: f64, // u128?
    /// Expected number of hashes required to produce the chain up to this block (in hex).
    pub chain_work: String,
    /// The number of transactions in the block.
    pub n_tx: u32,
    /// The hash of the previous block (if available).
    pub previous_block_hash: Option<BlockHash>,
    /// The hash of the next block (if available).
    pub next_block_hash: Option<BlockHash>,
}

/// Models the result of JSON-RPC method `gettxout`.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetTxOut {
    /// The hash of the block at the tip of the chain.
    pub best_block: BlockHash,
    /// The number of confirmations.
    pub confirmations: u64,
    /// The returned `TxOut` (strongly typed).
    pub tx_out: TxOut,
    /// Address that `tx_out` spends to.
    pub address: Address<NetworkUnchecked>,
    /// Coinbase or not.
    pub coinbase: bool,
}

/// Models the result of JSON-RPC method `getbestblockhash`.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBestBlockHash(pub BlockHash);
