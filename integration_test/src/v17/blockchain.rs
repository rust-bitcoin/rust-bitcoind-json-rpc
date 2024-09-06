// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing test methods on a JSON-RPC client.
//!
//! Specifically this is methods found under the `== Blockchain ==` section of the
//! API docs of `bitcoind v0.17.1`.

/// Requires `Client` to be in scope and to implement `get_best_block_hash`.
#[macro_export]
macro_rules! impl_test_v17__getbestblockhash {
    () => {
        fn best_block_hash() -> bitcoin::BlockHash {
            let bitcoind = $crate::bitcoind_no_wallet();
            bitcoind.client.best_block_hash().expect("best_block_hash failed")
        }

        #[test]
        fn get_best_block_hash() {
            let bitcoind = $crate::bitcoind_no_wallet();
            let json = bitcoind.client.get_best_block_hash().expect("getbestblockhash");
            assert!(json.into_model().is_ok());
        }
    };
}

/// Requires `Client` to be in scope and to implement `get_block 0`.
#[macro_export]
macro_rules! impl_test_v17__getblock_verbosity_0 {
    () => {
        #[test]
        fn get_block_verbosity_0() {
            let bitcoind = $crate::bitcoind_no_wallet();
            let block_hash = best_block_hash();

            let json = bitcoind.client.get_block_verbosity_zero(block_hash).expect("getblock 0");
            json.into_model().unwrap();
        }
    };
}

/// Requires `Client` to be in scope and to implement `get_block`.
#[macro_export]
macro_rules! impl_test_v17__getblock_verbosity_1 {
    () => {
        #[test]
        fn get_block_verbosity_1() {
            let bitcoind = $crate::bitcoind_no_wallet();
            let block_hash = best_block_hash();

            let json = bitcoind.client.get_block_verbosity_one(block_hash).expect("getblock 1");
            json.into_model().unwrap();
        }
    };
}

/// Requires `Client` to be in scope and to implement `get_block 2`.
#[macro_export]
macro_rules! impl_test_v17__getblock_verbosity_2 {
    () => {
        #[test]
        fn get_block_verbosity_2() {
            let bitcoind = $crate::bitcoind_no_wallet();
            let block_hash = best_block_hash();

            let json = client.get_block_verbosity_two(block_hash).expect("getblock 2");
            json.into_model().unwrap();
        }
    };
}

/// Requires `Client` to be in scope and to implement `get_blockchain_info`.
#[macro_export]
macro_rules! impl_test_v17__getblockchaininfo {
    () => {
        #[test]
        fn get_blockchain_info() {
            let bitcoind = $crate::bitcoind_no_wallet();
            let json = bitcoind.client.get_blockchain_info().expect("getblockchaininfo");
            assert!(json.into_model().is_ok());
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
