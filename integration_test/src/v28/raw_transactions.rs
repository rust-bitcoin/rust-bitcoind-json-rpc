// SPDX-License-Identifier: CC0-1.0

//! Macros for implementing test methods on a JSON-RPC client.
//!
//! Specifically this is methods found under the `== Rawtransactions ==` section of the
//! API docs of `bitcoind v28.0`.

/// Requires `Client` to be in scope
#[macro_export] macro_rules! impl_test_v28__submitpackage {
    () => {
        #[test]
        fn submitpackage() {
            // TODO
        }
    };
}
