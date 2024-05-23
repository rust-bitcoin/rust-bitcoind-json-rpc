// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing test methods on a JSON-RPC client.
//!
//! Specifically this is methods found under the `== Control ==` section of the
//! API docs of `bitcoind v0.17.1`.

/// Requires `Client` to be in scope and to implement `stop`.
#[macro_export]
macro_rules! impl_test_v17__stop {
    () => {
        #[test]
        fn get_blockchain_info() {
            let client = client();
            let _ = client.stop().expect("stop");
        }
    };
}
