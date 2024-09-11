// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing JSON-RPC methods on a client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of `bitcoind v0.17.1`.
//!
//! All macros require `Client` to be in scope.
//!
//! See or use the `define_jsonrpc_minreq_client!` macro to define a `Client`.

/// Implements bitcoind JSON-RPC API method `getbestblockhash`
#[macro_export]
macro_rules! impl_client_v17__getbestblockhash {
    () => {
        impl Client {
            /// Gets the blockhash of the current chain tip.
            pub fn best_block_hash(&self) -> Result<bitcoin::BlockHash> {
                let json = self.get_best_block_hash()?;
                Ok(json.block_hash()?)
            }

            pub fn get_best_block_hash(&self) -> Result<GetBestBlockHash> {
                self.call("getbestblockhash", &[])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getblock`
#[macro_export]
macro_rules! impl_client_v17__getblock {
    () => {
        impl Client {
            /// Gets a block by blockhash.
            pub fn get_block(&self, hash: BlockHash) -> Result<Block> {
                let json = self.get_block_verbosity_zero(hash)?;
                Ok(json.block()?)
            }

            pub fn get_block_verbosity_zero(
                &self,
                hash: BlockHash,
            ) -> Result<GetBlockVerbosityZero> {
                self.call("getblock", &[into_json(hash)?, 0.into()])
            }

            pub fn get_block_verbosity_one(&self, hash: BlockHash) -> Result<GetBlockVerbosityOne> {
                self.call("getblock", &[into_json(hash)?, 1.into()])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getblockchaininfo`
#[macro_export]
macro_rules! impl_client_v17__getblockchaininfo {
    () => {
        impl Client {
            pub fn get_blockchain_info(&self) -> Result<GetBlockchainInfo> {
                self.call("getblockchaininfo", &[])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getblockcount`
#[macro_export]
macro_rules! impl_client_v17__getblockcount {
    () => {
        impl Client {
            pub fn get_block_count(&self) -> Result<GetBlockCount> {
                self.call("getblockcount", &[])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getblockhash`
#[macro_export]
macro_rules! impl_client_v17__getblockhash {
    () => {
        impl Client {
            pub fn get_block_hash(&self, height: u64) -> Result<GetBlockHash> {
                self.call("getblockhash", &[into_json(height)?])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getblockheader`
#[macro_export]
macro_rules! impl_client_v17__getblockheader {
    () => {
        impl Client {
            pub fn get_block_header(&self, hash: &BlockHash) -> Result<GetBlockHeader> {
                self.call("getblockheader", &[into_json(hash)?, into_json(false)?])
            }

            // This is the same as calling getblockheader with verbose==true.
            pub fn get_block_header_verbose(
                &self,
                hash: &BlockHash,
            ) -> Result<GetBlockHeaderVerbose> {
                self.call("getblockheader", &[into_json(hash)?])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getblockstats`
#[macro_export]
macro_rules! impl_client_v17__getblockstats {
    () => {
        impl Client {
            pub fn get_block_stats_by_height(&self, height: u32) -> Result<GetBlockStats> {
                self.call("getblockstats", &[into_json(height)?])
            }

            pub fn get_block_stats_by_block_hash(&self, hash: &BlockHash) -> Result<GetBlockStats> {
                self.call("getblockstats", &[into_json(hash)?])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getchaintips`
#[macro_export]
macro_rules! impl_client_v17__getchaintips {
    () => {
        impl Client {
            pub fn get_chain_tips(&self) -> Result<GetChainTips> { self.call("getchaintips", &[]) }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getchaintxstats`
#[macro_export]
macro_rules! impl_client_v17__getchaintxstats {
    () => {
        impl Client {
            pub fn get_chain_tx_stats(&self) -> Result<GetChainTxStats> {
                self.call("getchaintxstats", &[])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getdifficulty`
#[macro_export]
macro_rules! impl_client_v17__getdifficulty {
    () => {
        impl Client {
            pub fn get_difficulty(&self) -> Result<GetDifficulty> {
                self.call("getdifficulty", &[])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `getmempoolancestors`
#[macro_export]
macro_rules! impl_client_v17__getmempoolancestors {
    () => {
        impl Client {
            pub fn get_mempool_ancestors(&self, txid: Txid) -> Result<GetMempoolAncestors> {
                self.call("getmempoolancestors", &[into_json(txid)?])
            }
        }
    };
}

/// Implements bitcoind JSON-RPC API method `gettxout`
#[macro_export]
macro_rules! impl_client_v17__gettxout {
    () => {
        impl Client {
            pub fn get_tx_out(&self, txid: Txid, vout: u64) -> Result<GetTxOut> {
                self.call("gettxout", &[into_json(txid)?, into_json(vout)?])
            }
        }
    };
}
