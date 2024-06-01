// SPDX-License-Identifier: CC0-1.0

//! Convert stdlib (version specific) types to concrete types.
//!
//! This module does the conversion for `v22.1` types to the general concrete types.

use core::fmt;
use std::collections::BTreeMap;

use bitcoin::{hex, network, Network, Work};
use internals::write_err;

use crate::json::v22;
use crate::model;

impl TryFrom<v22::GetBlockchainInfo> for model::GetBlockchainInfo {
    type Error = GetBlockchainInfoError;

    fn try_from(json: v22::GetBlockchainInfo) -> Result<Self, Self::Error> {
        use GetBlockchainInfoError as E;

        let chain = Network::from_core_arg(&json.chain).map_err(E::Chain)?;
        let best_block_hash = json.best_block_hash.parse().map_err(E::BestBlockHash)?;
        let chain_work = Work::from_unprefixed_hex(&json.chain_work).map_err(E::ChainWork)?;

        let mut softforks = BTreeMap::new();
        for (key, value) in json.softforks.into_iter() {
            softforks.insert(key, value.into());
        }

        Ok(Self {
            chain,
            blocks: json.blocks,
            headers: json.headers,
            best_block_hash,
            difficulty: json.difficulty, // FIXME: Should we convert to u128?
            median_time: json.median_time,
            verification_progress: json.verification_progress,
            initial_block_download: json.initial_block_download,
            chain_work,
            size_on_disk: json.size_on_disk,
            pruned: json.pruned,
            prune_height: json.prune_height,
            automatic_pruning: json.automatic_pruning,
            prune_target_size: json.prune_target_size,
            softforks,
            warnings: json.warnings,
        })
    }
}

/// Error when converting to a `v17::GetBlockchainInfo` type to a `concrete` type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetBlockchainInfoError {
    /// Conversion of the `chain` field failed.
    Chain(network::ParseNetworkError),
    /// Conversion of the `best_block_hash` field failed.
    BestBlockHash(hex::HexToArrayError),
    /// Conversion of the `chain_work` field failed.
    ChainWork(bitcoin::error::UnprefixedHexError),
}

impl fmt::Display for GetBlockchainInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetBlockchainInfoError::*;

        match *self {
            Chain(ref e) => write_err!(f, "conversion of the `chain` field failed"; e),
            BestBlockHash(ref e) => {
                write_err!(f, "conversion of the `best_block_hash` field failed"; e)
            }
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

impl From<v22::Softfork> for model::Softfork {
    fn from(json: v22::Softfork) -> Self {
        Self {
            type_: json.type_.into(),
            bip9: json.bip9.map(Into::into),
            height: json.height,
            active: json.active,
        }
    }
}

impl From<v22::SoftforkType> for model::SoftforkType {
    fn from(json: v22::SoftforkType) -> Self {
        match json {
            v22::SoftforkType::Buried => Self::Buried,
            v22::SoftforkType::Bip9 => Self::Bip9,
        }
    }
}

impl From<v22::Bip9SoftforkInfo> for model::Bip9SoftforkInfo {
    fn from(json: v22::Bip9SoftforkInfo) -> Self {
        Self {
            status: json.status.into(),
            bit: json.bit,
            start_time: json.start_time,
            timeout: json.timeout,
            since: json.since,
            statistics: json.statistics.map(Into::into),
        }
    }
}

impl From<v22::Bip9SoftforkStatus> for model::Bip9SoftforkStatus {
    fn from(json: v22::Bip9SoftforkStatus) -> Self {
        match json {
            v22::Bip9SoftforkStatus::Defined => Self::Defined,
            v22::Bip9SoftforkStatus::Started => Self::Started,
            v22::Bip9SoftforkStatus::LockedIn => Self::LockedIn,
            v22::Bip9SoftforkStatus::Active => Self::Active,
            v22::Bip9SoftforkStatus::Failed => Self::Failed,
        }
    }
}

impl From<v22::Bip9SoftforkStatistics> for model::Bip9SoftforkStatistics {
    fn from(json: v22::Bip9SoftforkStatistics) -> Self {
        Self {
            period: json.period,
            threshold: json.threshold,
            elapsed: json.elapsed,
            count: json.count,
            possible: json.possible,
        }
    }
}
