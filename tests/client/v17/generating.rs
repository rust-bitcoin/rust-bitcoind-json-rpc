// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing test methods on a JSON-RPC client.
//!
//! Specifically this is methods found under the `== Generating ==` section of the
//! API docs of `bitcoind v0.17.1`.

/// Requires `Client` to be in scope and to implement `get_blockchain_info`.
#[macro_export]
macro_rules! impl_test_v17__generatetoaddress {
    () => {
        #[test]
        fn generate_to_address() {
            let bitcoind = bitcoind_with_default_wallet();
            let address = bitcoind.client.new_address().expect("failed to get new address");
            let _ = bitcoind.client.generate_to_address(1, &address).expect("generatetoaddress");
        }
    };
}
