// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing test methods on a JSON-RPC client.
//!
//! Specifically this is methods found under the `== Network ==` section of the
//! API docs of `bitcoind v0.17.1`.

/// Requires `Client` to be in scope and to implement `get_network_info`.
#[macro_export]
macro_rules! impl_test_v17__getnetworkinfo {
    () => {
        #[test]
        fn get_network_info() {
            let bitcoind = bitcoind_no_wallet();
            let _ = bitcoind.client.get_network_info().expect("getnetworkinfo");
        }
    };
}
