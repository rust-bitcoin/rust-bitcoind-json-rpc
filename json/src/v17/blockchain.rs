// SPDX-License-Identifier: CC0-1.0

//! The JSON-RPC API for Bitcoin Core v0.17.1 - blockchain.
//!
//! Types for methods found under the `== Blockchain ==` section of the API docs.

use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use bitcoin::consensus::encode;
use bitcoin::error::UnprefixedHexError;
use bitcoin::{
    address, amount, block, hex, network, Address, Amount, Block, BlockHash, CompactTarget,
    Network, ScriptBuf, TxOut, Txid, Weight, Work,
};
use internals::write_err;
use serde::{Deserialize, Serialize};

use crate::model;

/// Result of JSON-RPC method `getbestblockhash`.
///
/// > getbestblockhash
/// >
/// > Returns the hash of the best (tip) block in the most-work fully-validated chain.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetBestBlockHash(pub String);

impl GetBestBlockHash {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBestBlockHash, hex::HexToArrayError> {
        let hash = self.0.parse::<BlockHash>()?;
        Ok(model::GetBestBlockHash(hash))
    }

    /// Converts json straight to a `bitcoin::BlockHash`.
    pub fn block_hash(self) -> Result<BlockHash, hex::HexToArrayError> { Ok(self.into_model()?.0) }
}

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

impl GetBlockchainInfo {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockchainInfo, GetBlockchainInfoError> {
        use GetBlockchainInfoError as E;

        let chain = Network::from_core_arg(&self.chain).map_err(E::Chain)?;
        let best_block_hash =
            self.best_block_hash.parse::<BlockHash>().map_err(E::BestBlockHash)?;
        // FIXME: Is unprefixed correct?
        let chain_work = Work::from_unprefixed_hex(&self.chain_work).map_err(E::ChainWork)?;

        let softforks = BTreeMap::new(); // TODO: Handle softforks stuff.

        Ok(model::GetBlockchainInfo {
            chain,
            blocks: self.blocks,
            headers: self.headers,
            best_block_hash,
            difficulty: self.difficulty,
            median_time: self.median_time,
            verification_progress: self.verification_progress,
            initial_block_download: self.initial_block_download,
            chain_work,
            size_on_disk: self.size_on_disk,
            pruned: self.pruned,
            prune_height: self.prune_height,
            automatic_pruning: self.automatic_pruning,
            prune_target_size: self.prune_target_size,
            softforks,
            warnings: self.warnings,
        })
    }
}

// FIXME: Me mightn't need this.
impl Bip9SoftforkStatus {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> model::Bip9SoftforkStatus {
        use model::Bip9SoftforkStatus::*;

        match self {
            Self::Defined => Defined,
            Self::Started => Started,
            Self::LockedIn => LockedIn,
            Self::Active => Active,
            Self::Failed => Failed,
        }
    }
}

/// Error when converting a `GetBlockchainInfo` type into the model type.
#[derive(Debug)]
pub enum GetBlockchainInfoError {
    Chain(network::ParseNetworkError),
    BestBlockHash(hex::HexToArrayError),
    ChainWork(UnprefixedHexError),
}

impl fmt::Display for GetBlockchainInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetBlockchainInfoError::*;

        match *self {
            Chain(ref e) => write_err!(f, "conversion of the `chain` field failed"; e),
            BestBlockHash(ref e) =>
                write_err!(f, "conversion of the `best_block_hash` field failed"; e),
            ChainWork(ref e) => write_err!(f, "conversion of the `chain_work` field failed"; e),
        }
    }
}

impl std::error::Error for GetBlockchainInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetBlockchainInfoError::*;

        match *self {
            Chain(ref e) => Some(e),
            BestBlockHash(ref e) => Some(e),
            ChainWork(ref e) => Some(e),
        }
    }
}

/// Result of JSON-RPC method `getblock` with verbosity set to 0.
///
/// A string that is serialized, hex-encoded data for block 'hash'.
///
/// Method call: `getblock "blockhash" ( verbosity )`
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct GetBlockVerbosityZero(pub String);

impl GetBlockVerbosityZero {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockVerbosityZero, encode::FromHexError> {
        let block = encode::deserialize_hex(&self.0)?;
        Ok(model::GetBlockVerbosityZero(block))
    }

    /// Converts json straight to a `bitcoin::Block`.
    pub fn block(self) -> Result<Block, encode::FromHexError> { Ok(self.into_model()?.0) }
}

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
    #[serde(rename = "strippedsize")]
    pub stripped_size: Option<usize>,
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
    #[serde(rename = "merkleroot")]
    pub merkle_root: String,
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

impl GetBlockVerbosityOne {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetBlockVerbosityOne, GetBlockVerbosityOneError> {
        use GetBlockVerbosityOneError as E;

        let hash = self.hash.parse::<BlockHash>().map_err(E::Hash)?;
        let weight = Weight::from_wu(self.weight); // TODO: Confirm this uses weight units.
        let version = block::Version::from_consensus(self.version);

        // FIXME: Is there a better way to handle the error without type annotations on `collect`?
        let tx = self
            .tx
            .iter()
            .map(|t| encode::deserialize_hex::<Txid>(t).map_err(E::Tx))
            .collect::<Result<Vec<_>, _>>()?;

        // FIXME: Is unprefixed correct?
        let bits = CompactTarget::from_unprefixed_hex(&self.bits).map_err(E::Bits)?;
        let chain_work = Work::from_unprefixed_hex(&self.chain_work).map_err(E::ChainWork)?;

        let previous_block_hash = match self.previous_block_hash {
            Some(hash) => Some(hash.parse::<BlockHash>().map_err(E::PreviousBlockHash)?),
            None => None,
        };
        let next_block_hash = match self.next_block_hash {
            Some(hash) => Some(hash.parse::<BlockHash>().map_err(E::NextBlockHash)?),
            None => None,
        };

        Ok(model::GetBlockVerbosityOne {
            hash,
            confirmations: self.confirmations,
            size: self.size,
            stripped_size: self.stripped_size,
            weight,
            height: self.height,
            version,
            version_hex: self.version_hex,
            merkle_root: self.merkle_root, // TODO: Use hash, which one depends on segwit or not
            tx,
            time: self.time, // TODO: Use stronger type.
            median_time: self.median_time,
            nonce: self.nonce,
            bits,
            difficulty: self.difficulty,
            chain_work,
            n_tx: self.n_tx,
            previous_block_hash,
            next_block_hash,
        })
    }
}

/// Error when converting a `GetBlockVerbasityOne` type into the model type.
#[derive(Debug)]
pub enum GetBlockVerbosityOneError {
    /// Conversion of the transaction `hash` field failed.
    Hash(hex::HexToArrayError),
    /// Conversion of the transaction `hex` field failed.
    Tx(encode::FromHexError),
    /// Conversion of the transaction `bits` field failed.
    Bits(UnprefixedHexError),
    /// Conversion of the transaction `chain_work` field failed.
    ChainWork(UnprefixedHexError),
    /// Conversion of the transaction `previous_block_hash` field failed.
    PreviousBlockHash(hex::HexToArrayError),
    /// Conversion of the transaction `next_block_hash` field failed.
    NextBlockHash(hex::HexToArrayError),
}

impl fmt::Display for GetBlockVerbosityOneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetBlockVerbosityOneError::*;

        match *self {
            Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            Tx(ref e) => write_err!(f, "conversion of the `tx` field failed"; e),
            Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            ChainWork(ref e) => write_err!(f, "conversion of the `chain_ork` field failed"; e),
            PreviousBlockHash(ref e) =>
                write_err!(f, "conversion of the `previous_block_hash` field failed"; e),
            NextBlockHash(ref e) =>
                write_err!(f, "conversion of the `next_block_hash` field failed"; e),
        }
    }
}

impl std::error::Error for GetBlockVerbosityOneError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetBlockVerbosityOneError::*;

        match *self {
            Hash(ref e) => Some(e),
            Tx(ref e) => Some(e),
            Bits(ref e) => Some(e),
            ChainWork(ref e) => Some(e),
            PreviousBlockHash(ref e) => Some(e),
            NextBlockHash(ref e) => Some(e),
        }
    }
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
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct GetTxOut {
    /// The hash of the block at the tip of the chain.
    #[serde(rename = "bestblock")]
    pub best_block: String,
    /// The number of confirmations.
    pub confirmations: u32,
    /// The transaction value in BTC.
    pub value: f64,
    /// The script pubkey.
    #[serde(rename = "scriptPubkey")]
    pub script_pubkey: ScriptPubkey,
    /// Coinbase or not.
    pub coinbase: bool,
}

/// A script pubkey.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ScriptPubkey {
    /// Script assembly.
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

impl GetTxOut {
    /// Converts version specific type to a version in-specific, more strongly typed type.
    pub fn into_model(self) -> Result<model::GetTxOut, GetTxOutError> {
        use GetTxOutError as E;

        let best_block = self.best_block.parse::<BlockHash>().map_err(E::BestBlock)?;

        let tx_out = TxOut {
            value: Amount::from_btc(self.value).map_err(E::Value)?,
            script_pubkey: ScriptBuf::from_hex(&self.script_pubkey.hex).map_err(E::ScriptPubkey)?,
        };

        let address = Address::from_str(&self.script_pubkey.address).map_err(E::Address)?;

        Ok(model::GetTxOut {
            best_block,
            confirmations: self.confirmations,
            tx_out,
            address,
            coinbase: self.coinbase,
        })
    }
}

/// Error when converting a `GetTxOut` type into the model type.
#[derive(Debug)]
pub enum GetTxOutError {
    /// Conversion of the transaction `best_block` field failed.
    BestBlock(hex::HexToArrayError),
    /// Conversion of the transaction `value` field failed.
    Value(amount::ParseAmountError),
    /// Conversion of the transaction `script_pubkey` field failed.
    ScriptPubkey(hex::HexToBytesError),
    /// Conversion of the transaction `address` field failed.
    Address(address::ParseError),
}

impl fmt::Display for GetTxOutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetTxOutError::*;

        match *self {
            BestBlock(ref e) => write_err!(f, "conversion of the `best_block` field failed"; e),
            Value(ref e) => write_err!(f, "conversion of the `value` field failed"; e),
            ScriptPubkey(ref e) =>
                write_err!(f, "conversion of the `script_pubkey` field failed"; e),
            Address(ref e) => write_err!(f, "conversion of the `address` field failed"; e),
        }
    }
}

impl std::error::Error for GetTxOutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetTxOutError::*;

        match *self {
            BestBlock(ref e) => Some(e),
            Value(ref e) => Some(e),
            ScriptPubkey(ref e) => Some(e),
            Address(ref e) => Some(e),
        }
    }
}
