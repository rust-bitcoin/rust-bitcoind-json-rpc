// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing test methods on a JSON-RPC client.
//!
//! Specifically this is methods found under the `== Generating ==` section of the
//! API docs of `bitcoind v0.17.1`.

/// Requires `Client` to be in scope and to implement `generate_to_address`.
#[macro_export]
macro_rules! impl_test_v17__generatetoaddress {
    () => {
        #[test]
        fn generate_to_address() {
            const NBLOCKS: usize = 1;

            let bitcoind = $crate::bitcoind_with_default_wallet();
            let address = bitcoind.client.new_address().expect("failed to get new address");
            let json = bitcoind.client.generate_to_address(NBLOCKS, &address).expect("generatetoaddress");
            json.into_model().unwrap();
        }
    };
}

/// Requires `Client` to be in scope and to implement `generate`.
#[macro_export]
macro_rules! impl_test_v17__generate {
    () => {
        #[test]
        fn generate() {
            const NBLOCKS: usize = 100;

            let bitcoind = $crate::bitcoind_with_default_wallet();
            let json = bitcoind.client.generate(NBLOCKS).expect("generate");
            json.into_model().unwrap();
        }
    };
}
