// SPDX-License-Identifier: CC0-1.0

//! Convert stdlib (version specific) types to concrete types.
//!
//! This module does the conversion for `v0.17.1` types to the general concrete types.

use core::fmt;
use core::str::FromStr;

use bitcoin::consensus::encode;
use bitcoin::{
    address, amount, block, hex, Address, Amount, BlockHash, CompactTarget, ScriptBuf, TxOut,
    Weight,
};
use internals::write_err;

use crate::v17;
use crate::model;

impl TryFrom<v17::GetBlockchainInfo> for model::GetBlockchainInfo {
    type Error = ();

    fn try_from(_: v17::GetBlockchainInfo) -> Result<Self, Self::Error> {
        todo!("softfork fields have changed considerably by v22")
    }
}

impl TryFrom<v17::GetBlockVerbosityZero> for model::GetBlockVerbosityZero {
    type Error = encode::FromHexError;

    fn try_from(json: v17::GetBlockVerbosityZero) -> Result<Self, Self::Error> {
        let header = encode::deserialize_hex(&json.0)?;
        Ok(Self(header))
    }
}

impl TryFrom<v17::GetBlockVerbosityOne> for model::GetBlockVerbosityOne {
    type Error = GetBlockVerbosityOneError;

    fn try_from(json: v17::GetBlockVerbosityOne) -> Result<Self, Self::Error> {
        use GetBlockVerbosityOneError as E;

        let hash = json.hash.parse().map_err(E::Hash)?;
        let weight = Weight::from_wu(json.weight); // FIXME: Is this correct?
        let version = block::Version::from_consensus(json.version);
        let bits = CompactTarget::from_unprefixed_hex(&json.bits).map_err(E::Bits)?;
        let previous_block_hash = match json.previous_block_hash {
            Some(hex) => Some(hex.parse().map_err(E::PreviousBlockHash)?),
            None => None,
        };
        let next_block_hash = match json.next_block_hash {
            Some(hex) => Some(hex.parse().map_err(E::NextBlockHash)?),
            None => None,
        };

        Ok(Self {
            hash,
            confirmations: json.confirmations,
            size: json.size,
            strippedsize: json.strippedsize,
            weight,
            height: json.height,
            version,
            version_hex: json.version_hex,
            merkleroot: json.merkleroot,
            tx: json.tx,
            time: json.time,
            median_time: json.median_time,
            nonce: json.nonce,
            bits,
            difficulty: json.difficulty, // u128?
            chain_work: json.chain_work,
            n_tx: json.n_tx,
            previous_block_hash,
            next_block_hash,
        })
    }
}

/// Error when converting to a `v17::GetBlock` type to a `concrete` type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetBlockVerbosityOneError {
    /// Conversion of the `hash` field failed.
    Hash(hex::HexToArrayError),
    /// Conversion of the `bits` field failed.
    Bits(bitcoin::error::UnprefixedHexError),
    /// Conversion of the `previous_block_hash` field failed.
    PreviousBlockHash(hex::HexToArrayError),
    /// Conversion of the `next_block_hash` field failed.
    NextBlockHash(hex::HexToArrayError),
}

impl fmt::Display for GetBlockVerbosityOneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetBlockVerbosityOneError::*;

        match *self {
            Hash(ref e) => write_err!(f, "conversion of the `hash` field failed"; e),
            Bits(ref e) => write_err!(f, "conversion of the `bits` field failed"; e),
            PreviousBlockHash(ref e) => {
                write_err!(f, "conversion of the `previous_block_hash` field failed"; e)
            }
            NextBlockHash(ref e) => {
                write_err!(f, "conversion of the `next_block_hash` field failed"; e)
            }
        }
    }
}

impl std::error::Error for GetBlockVerbosityOneError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetBlockVerbosityOneError::*;

        match *self {
            Hash(ref e) => Some(e),
            Bits(ref e) => Some(e),
            PreviousBlockHash(ref e) => Some(e),
            NextBlockHash(ref e) => Some(e),
        }
    }
}

impl TryFrom<v17::GetTxOut> for model::GetTxOut {
    type Error = GetTxOutError;

    fn try_from(json: v17::GetTxOut) -> Result<Self, Self::Error> {
        use GetTxOutError as E;

        let best_block = json.best_block.parse().map_err(E::BestBlock)?;

        let value = Amount::from_sat(json.value);

        // TODO: We could parse `asm` as well and sanity check it matches the hex?
        let script_pubkey = ScriptBuf::from_hex(&json.script_pubkey.hex).map_err(E::Hex)?;

        // TODO: We could parse the `type_` as well and sanity check it matches the `Address::address_type()`?
        let address = Address::from_str(&json.script_pubkey.address).map_err(E::Address)?;

        Ok(Self {
            best_block,
            confirmations: json.confirmations,
            tx_out: TxOut { value, script_pubkey },
            address,
            coinbase: json.coinbase,
        })
    }
}

/// Error when converting to a `v17::GetTxOut` type to a `concrete` type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GetTxOutError {
    /// Conversion of the `best_block` field failed.
    BestBlock(hex::HexToArrayError),
    /// Conversion of the `value` field failed.
    Value(amount::ParseAmountError),
    /// Conversion of the `script_pubkey.hex` field failed.
    Hex(hex::HexToBytesError),
    /// Conversion of the `script_pubkey.address` field failed.
    Address(address::ParseError),
}

impl fmt::Display for GetTxOutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use GetTxOutError::*;

        match *self {
            BestBlock(ref e) => write_err!(f, "conversion of the `best_block` field failed"; e),
            Value(ref e) => write_err!(f, "conversion of the `value` field failed"; e),
            Hex(ref e) => {
                write_err!(f, "conversion of the `script_pubkey.hex` field failed"; e)
            }
            Address(ref e) => {
                write_err!(f, "conversion of the `script_pubkey.address` field failed"; e)
            }
        }
    }
}

impl std::error::Error for GetTxOutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use GetTxOutError::*;

        match *self {
            BestBlock(ref e) => Some(e),
            Value(ref e) => Some(e),
            Hex(ref e) => Some(e),
            Address(ref e) => Some(e),
        }
    }
}

impl TryFrom<v17::GetBestBlockHash> for model::GetBestBlockHash {
    type Error = hex::HexToArrayError;

    fn try_from(json: v17::GetBestBlockHash) -> Result<Self, Self::Error> {
        let block_hash = BlockHash::from_str(&json.0)?;
        Ok(Self(block_hash))
    }
}
