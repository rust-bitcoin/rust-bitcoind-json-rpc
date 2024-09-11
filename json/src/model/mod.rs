// SPDX-License-Identifier: CC0-1.0

//! Models of the data returned by the JSON-RPC API of Bitcoin Core.
//!
//! The types here model the data returned by Bitcoin Core in a version non-specific way. In other
//! words one can use a particular `bitcoind` version via the version specific module (e.g.
//! `crate::v26`) then convert the `json` types to one of the modelled types in this module using
//! `TryFrom`.

// Separated by the section name in the `bitcoind` JSON-RPC docs.
mod blockchain;
mod control;
mod generating;
mod mining;
mod network;
mod raw_transactions;
mod util;
mod wallet;
mod zmq;

// TODO/QUESTIONS
//
// - Should all types here be non_exhaustive (otherwise evertime Core changes them we will need a new major release)?
// - Should we provide an inherent convertion method because try_from's lack of type inference is annoying to use.

#[doc(inline)]
pub use self::{
    blockchain::{
        Bip9SoftforkInfo, Bip9SoftforkStatistics, Bip9SoftforkStatus, ChainTips, ChainTipsStatus,
        GetBestBlockHash, GetBlockCount, GetBlockHash, GetBlockHeader, GetBlockHeaderVerbose,
        GetBlockStats, GetBlockVerbosityOne, GetBlockVerbosityZero, GetBlockchainInfo,
        GetChainTips, GetChainTxStats, GetDifficulty, GetMempoolAncestors,
        GetMempoolAncestorsVerbose, GetTxOut, Softfork, SoftforkType,
    },
    generating::{Generate, GenerateToAddress},
    network::{GetNetworkInfo, GetNetworkInfoAddress, GetNetworkInfoNetwork},
    raw_transactions::SendRawTransaction,
    wallet::{
        CreateWallet, GetBalance, GetBalances, GetBalancesMine, GetBalancesWatchOnly,
        GetNewAddress, GetTransaction, GetTransactionDetail, GetTransactionDetailCategory,
        LoadWallet, SendToAddress, UnloadWallet,
    },
};
