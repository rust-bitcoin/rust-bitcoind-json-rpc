// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing test methods on a JSON-RPC client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of `bitcoind v0.17.1`.

/// Requires `Client` to be in scope and to implement `get_blockchain_info`.
#[macro_export]
macro_rules! impl_test_v17__getblockchaininfo {
    () => {
        #[test]
        fn get_blockchain_info() {
            let client = client();
            let _ = client.get_blockchain_info().expect("getblockchaininfo");
        }
    };
}

/// Requires `Client` to be in scope and to implement `get_best_block_hash`.
#[macro_export]
macro_rules! impl_test_v17__getbestblockhash {
    () => {
        fn best_block_hash() -> bitcoin::BlockHash {
            let client = client();
            client.best_block_hash().expect("best_block_hash failed")
        }

        #[test]
        fn get_best_block_hash() {
            let client = client();
            let _ = client.get_best_block_hash().expect("getbestblockhash");
        }
    };
}

/// Requires `Client` to be in scope and to implement `get_block`.
#[macro_export]
macro_rules! impl_test_v17__getblock {
    () => {
        #[test]
        fn get_block() {
            let client = client();
            let block_hash = best_block_hash();

            let _ = client.get_block_verbosity_zero(&block_hash).expect("getblock 0");
            let _ = client.get_block_verbosity_one(&block_hash).expect("getblock 1");
            // TODO: getblock 2
            // let json = client.get_block_verbosity_two(&block_hash).expect("getblock 2");
        }
    };
}

/// Requires `Client` to be in scope and to implement `get_tx_out`.
#[macro_export]
macro_rules! impl_test_v17__gettxout {
    () => {
        #[test]
        fn get_tx_out() { todo!() }
    };
}
