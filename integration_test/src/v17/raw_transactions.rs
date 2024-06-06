// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing test methods on a JSON-RPC client.
//!
//! Specifically this is methods found under the `== Rawtransactions ==` section of the
//! API docs of `bitcoind v0.17.1`.

/// Requires `Client` to be in scope and to implement `get_best_block_hash`.
#[macro_export]
macro_rules! impl_test_v17__sendrawtransaction {
    () => {
        #[test]
        fn send_raw_transaction() {
            // let bitcoind = $crate::bitcoind_no_wallet();
            // // TODO: Get a transaction from somewhere and send it.
            // let _ = bitcoind.client.get_best_block_hash().expect("getbestblockhash");
        }
    };
}
