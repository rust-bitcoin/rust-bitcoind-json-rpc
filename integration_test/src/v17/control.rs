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
        fn stop() {
            let bitcoind = $crate::bitcoind_no_wallet();
            let _ = bitcoind.client.stop().expect("stop");
        }
    };
}
