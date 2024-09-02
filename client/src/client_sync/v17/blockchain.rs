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
            pub fn get_block(&self, hash: &BlockHash) -> Result<Block> {
                let json = self.get_block_verbosity_zero(hash)?;
                Ok(json.block()?)
            }

            // FIXME(getblock): This handling of optional args is ugly as hell but because the returned json
            // is different for each verbosity these are functionally different methods. Is there a better way?

            pub fn get_block_verbosity_zero(
                &self,
                hash: &BlockHash,
            ) -> Result<GetBlockVerbosityZero> {
                self.call("getblock", &[into_json(hash)?, 0.into()])
            }

            pub fn get_block_verbosity_one(
                &self,
                hash: &BlockHash,
            ) -> Result<GetBlockVerbosityOne> {
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
